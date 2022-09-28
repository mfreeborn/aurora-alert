# Aurora Alert

A full stack web app written in Rust which allows users to subscribe to email alerts for aurora events. Users are also provided with the current weather status and cloud cover at their chosen locations.

## Deployment

From the cargo workspace root, run `./scripts/deploy.sh`, which will produce a `dist/` directory containing the necessary artifacts for deploying to a Raspberry Pi.

Move that directory to the Raspberry Pi e.g. `scp -r dist/ {pi_username}@{pi_ip_address}:~/.aurora-alert`.

On the Pi, the app up as a `systems` service:

```bash
$ sudo mv aurora-alert.service /etc/systemd/system/
$ sudo systemctl daemon-reload
$ sudo systemctl start aurora-alert.service
$ sudo systemctl enable aurora-alert.service
```

Assuming the default configuration is unchanged, the app is now live at http://{pi_address}:9090.