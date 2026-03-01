# Native Rust Play Store Upload

An action to upload and publish an Android app build (AAB or APK) to Google Play. 
This action uses a native Rust binary to perform the upload without requiring Docker or Node.js.

## Usage

```yaml
jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v6
      
      - name: Upload to Google Play
        uses: caffeine-soft/google-play-upload@v0
        with:
          packageName: 'com.example.app'
          track: 'production'
          status: 'completed'
          releaseFile: 'app/build/outputs/bundle/release/app-release.aab'
          serviceAccountJsonPlainText: ${{ secrets.SERVICE_ACCOUNT_JSON }}
```

## Inputs

| Name | Description | Required | Default |
| ---- | ----------- | -------- | ------- |
| `packageName` | Android package name | `true` | |
| `track` | Release track (e.g. production, internal) | `true` | |
| `status` | Release status (e.g. completed, draft, inProgress) | `false` | `completed` |
| `releaseFile` | Path to .aab or .apk | `true` | |
| `serviceAccountJsonPlainText` | GCP Service Account JSON | `true` | |
