# mytz
A web service that gets my timezone. For an example see https://tz.chills.dev/. It may or may not be up.

- [/](https://tz.chills.dev/) returns your timezone in `Africa/Johannesburg` format.
- [/tz](https://tz.chills.dev/tz) returns your timezone in `SAST-2` format (which is useful if you want to set the `TZ` environment variable, or [set the timezone on an ESP-32](https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/system_time.html#timezones)).
