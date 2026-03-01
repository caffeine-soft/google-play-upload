use anyhow::{Context, Result};
use google_androidpublisher3::{AndroidPublisher, api, hyper_rustls, hyper_util, oauth2};
use std::env;
use std::fs;

#[tokio::main]
async fn main() -> Result<()> {
    //  INPUTS
    let package_name = env::var("INPUT_PACKAGENAME")?;
    let track = env::var("INPUT_TRACK")?;
    let status = env::var("INPUT_STATUS").unwrap_or_else(|_| "completed".to_string());
    let service_account_json = env::var("INPUT_SERVICEACCOUNTJSONPLAINTEXT")?;

    // Auth
    let secret = oauth2::parse_service_account_key(service_account_json)?;
    let auth = oauth2::ServiceAccountAuthenticator::builder(secret)
        .build()
        .await?;

    let client = hyper_util::client::legacy::Client::builder(hyper_util::rt::TokioExecutor::new())
        .build(
            hyper_rustls::HttpsConnectorBuilder::new()
                .with_native_roots()?
                .https_or_http()
                .enable_http1()
                .build(),
        );
    let hub = AndroidPublisher::new(client, auth);

    // Edit
    let (_, edit) = hub
        .edits()
        .insert(api::AppEdit::default(), &package_name)
        .doit()
        .await?;
    let edit_id = edit.id.context("No Edit ID")?;

    // release file
    let release_file = env::var("INPUT_RELEASEFILE")?;
    let file = fs::File::open(&release_file)?;
    let (_, bundle) = hub
        .edits()
        .bundles_upload(&package_name, &edit_id)
        .upload(file, "application/octet-stream".parse().unwrap())
        .await?;

    let version_code = bundle.version_code.context("No version code")?;

    // release track
    let release = api::TrackRelease {
        version_codes: Some(vec![version_code as i64]),
        status: Some(status),
        ..Default::default()
    };

    let track_update = api::Track {
        track: Some(track.clone()),
        releases: Some(vec![release]),
        ..Default::default()
    };

    hub.edits()
        .tracks_update(track_update, &package_name, &edit_id, &track)
        .doit()
        .await?;

    // Commit
    hub.edits().commit(&package_name, &edit_id).doit().await?;

    println!(
        "Successfully committed release with version {}",
        version_code
    );
    Ok(())
}
