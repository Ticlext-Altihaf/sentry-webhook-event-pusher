use actix_web::{middleware, web, App, HttpServer, Responder};
extern crate dotenv;
use chrono::{DateTime, Utc};
use dotenv::dotenv;
use std::env;
mod sentry_definition;
/**
 * {"id": "5461763487", "project": "berani-learning-web", "project_name": "berani-learning-web", "project_slug": "berani-learning-web", "logger": null, "level": "error", "culprit": "raven.scripts.runner in main", "message": "This is an example Python exception", "url": "https://berani.sentry.io/issues/5461763487/?referrer=webhooks_plugin", "triggering_rules": [], "event": {"event_id": "a1fdd396763241eda55f7e42178c4ed4", "level": "error", "version": "5", "type": "default", "logentry": {"formatted": "This is an example Python exception", "message": null, "params": null}, "logger": "", "modules": {"my.package": "1.0.0"}, "platform": "python", "timestamp": 1717749612.583, "received": 1717749672.5841, "environment": "prod", "user": {"id": "1", "email": "sentry@example.com", "ip_address": "127.0.0.1", "username": "sentry", "name": "Sentry", "geo": {"country_code": "AU", "city": "Melbourne", "region": "VIC"}, "sentry_user": "id:1"}, "request": {"url": "http://example.com/foo", "method": "GET", "data": {"hello": "world"}, "query_string": [["foo", "bar"]], "cookies": [["foo", "bar"], ["biz", "baz"]], "headers": [["Content-Type", "application/json"], ["Referer", "http://example.com"], ["User-Agent", "Mozilla/5.0 (Windows NT 6.2; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/28.0.1500.72 Safari/537.36"]], "env": {"ENV": "prod"}, "inferred_content_type": "application/json", "api_target": null, "fragment": null}, "contexts": {"browser": {"name": "Chrome", "version": "28.0.1500", "type": "browser"}, "client_os": {"name": "Windows", "version": "8", "type": "os"}}, "stacktrace": {"frames": [{"function": "build_msg", "module": "raven.base", "filename": "raven/base.py", "abs_path": "/home/ubuntu/.virtualenvs/getsentry/src/raven/raven/base.py", "lineno": 303, "pre_context": ["                frames = stack", "", "            data.update({", "                'sentry.interfaces.Stacktrace': {", "                    'frames': get_stack_info(frames,"], "context_line": "                        transformer=self.transform)", "post_context": ["                },", "            })", "", "        if 'sentry.interfaces.Stacktrace' in data:", "            if self.include_paths:"], "in_app": false, "vars": {"'culprit'": null, "'data'": {"'message'": "u'This is a test message generated using ``raven test``'", "'sentry.interfaces.Message'": {"'message'": "u'This is a test message generated using ``raven test``'", "'params'": []}}, "'date'": "datetime.datetime(2013, 8, 13, 3, 8, 24, 880386)", "'event_id'": "'54a322436e1b47b88e239b78998ae742'", "'event_type'": "'raven.events.Message'", "'extra'": {"'go_deeper'": [["{\"'bar'\":[\"'baz'\"],\"'foo'\":\"'bar'\"}"]], "'loadavg'": [0.37255859375, 0.5341796875, 0.62939453125], "'user'": "'dcramer'"}, "'frames'": "<generator object iter_stack_frames at 0x107bcc3c0>", "'handler'": "<raven.events.Message object at 0x107bd0890>", "'k'": "'sentry.interfaces.Message'", "'kwargs'": {"'level'": 20, "'message'": "'This is a test message generated using ``raven test``'"}, "'public_key'": null, "'result'": {"'message'": "u'This is a test message generated using ``raven test``'", "'sentry.interfaces.Message'": {"'message'": "u'This is a test message generated using ``raven test``'", "'params'": []}}, "'self'": "<raven.base.Client object at 0x107bb8210>", "'stack'": true, "'tags'": null, "'time_spent'": null, "'v'": {"'message'": "u'This is a test message generated using ``raven test``'", "'params'": []}}, "colno": null, "data": null, "errors": null, "raw_function": null, "image_addr": null, "instruction_addr": null, "addr_mode": null, "package": null, "platform": null, "source_link": null, "symbol": null, "symbol_addr": null, "trust": null, "snapshot": null, "lock": null}, {"function": "capture", "module": "raven.base", "filename": "raven/base.py", "abs_path": "/home/ubuntu/.virtualenvs/getsentry/src/raven/raven/base.py", "lineno": 459, "pre_context": ["        if not self.is_enabled():", "            return", "", "        data = self.build_msg(", "            event_type, data, date, time_spent, extra, stack, tags=tags,"], "context_line": "            **kwargs)", "post_context": ["", "        self.send(**data)", "", "        return (data.get('event_id'),)", ""], "in_app": false, "vars": {"'data'": null, "'date'": null, "'event_type'": "'raven.events.Message'", "'extra'": {"'go_deeper'": [["{\"'bar'\":[\"'baz'\"],\"'foo'\":\"'bar'\"}"]], "'loadavg'": [0.37255859375, 0.5341796875, 0.62939453125], "'user'": "'dcramer'"}, "'kwargs'": {"'level'": 20, "'message'": "'This is a test message generated using ``raven test``'"}, "'self'": "<raven.base.Client object at 0x107bb8210>", "'stack'": true, "'tags'": null, "'time_spent'": null}, "colno": null, "data": null, "errors": null, "raw_function": null, "image_addr": null, "instruction_addr": null, "addr_mode": null, "package": null, "platform": null, "source_link": null, "symbol": null, "symbol_addr": null, "trust": null, "snapshot": null, "lock": null}, {"function": "captureMessage", "module": "raven.base", "filename": "raven/base.py", "abs_path": "/home/ubuntu/.virtualenvs/getsentry/src/raven/raven/base.py", "lineno": 577, "pre_context": ["        \"\"\"", "        Creates an event from ``message``.", "", "        >>> client.captureMessage('My event just happened!')", "        \"\"\""], "context_line": "        return self.capture('raven.events.Message', message=message, **kwargs)", "post_context": ["", "    def captureException(self, exc_info=None, **kwargs):", "        \"\"\"", "        Creates an event from an exception.", ""], "in_app": false, "vars": {"'kwargs'": {"'data'": null, "'extra'": {"'go_deeper'": ["[{\"'bar'\":[\"'baz'\"],\"'foo'\":\"'bar'\"}]"], "'loadavg'": [0.37255859375, 0.5341796875, 0.62939453125], "'user'": "'dcramer'"}, "'level'": 20, "'stack'": true, "'tags'": null}, "'message'": "'This is a test message generated using ``raven test``'", "'self'": "<raven.base.Client object at 0x107bb8210>"}, "colno": null, "data": null, "errors": null, "raw_function": null, "image_addr": null, "instruction_addr": null, "addr_mode": null, "package": null, "platform": null, "source_link": null, "symbol": null, "symbol_addr": null, "trust": null, "snapshot": null, "lock": null}, {"function": "send_test_message", "module": "raven.scripts.runner", "filename": "raven/scripts/runner.py", "abs_path": "/home/ubuntu/.virtualenvs/getsentry/src/raven/raven/scripts/runner.py", "lineno": 77, "pre_context": ["        level=logging.INFO,", "        stack=True,", "        tags=options.get('tags', {}),", "        extra={", "            'user': get_uid(),"], "context_line": "            'loadavg': get_loadavg(),", "post_context": ["        },", "    ))", "", "    if client.state.did_fail():", "        print('error!')"], "in_app": false, "vars": {"'client'": "<raven.base.Client object at 0x107bb8210>", "'data'": null, "'k'": "'secret_key'", "'options'": {"'data'": null, "'tags'": null}}, "colno": null, "data": null, "errors": null, "raw_function": null, "image_addr": null, "instruction_addr": null, "addr_mode": null, "package": null, "platform": null, "source_link": null, "symbol": null, "symbol_addr": null, "trust": null, "snapshot": null, "lock": null}, {"function": "main", "module": "raven.scripts.runner", "filename": "raven/scripts/runner.py", "abs_path": "/home/ubuntu/.virtualenvs/getsentry/src/raven/raven/scripts/runner.py", "lineno": 112, "pre_context": ["    print(\"Using DSN configuration:\")", "    print(\" \", dsn)", "    print()", "", "    client = Client(dsn, include_paths=['raven'])"], "context_line": "    send_test_message(client, opts.__dict__)", "in_app": false, "vars": {"'args'": ["'test'", "'https://ebc35f33e151401f9deac549978bda11:f3403f81e12e4c24942d505f086b2cad@sentry.io/1'"], "'client'": "<raven.base.Client object at 0x107bb8210>", "'dsn'": "'https://ebc35f33e151401f9deac549978bda11:f3403f81e12e4c24942d505f086b2cad@sentry.io/1'", "'opts'": "<Values at 0x107ba3b00: {'data': None, 'tags': None}>", "'parser'": "<optparse.OptionParser instance at 0x107ba3368>", "'root'": "<logging.Logger object at 0x107ba5b10>"}, "colno": null, "data": null, "errors": null, "raw_function": null, "image_addr": null, "instruction_addr": null, "addr_mode": null, "package": null, "platform": null, "post_context": null, "source_link": null, "symbol": null, "symbol_addr": null, "trust": null, "snapshot": null, "lock": null}]}, "tags": [["browser", "Chrome 28.0.1500"], ["browser.name", "Chrome"], ["client_os", "Windows 8"], ["client_os.name", "Windows"], ["environment", "prod"], ["level", "error"], ["sentry:user", "id:1"], ["server_name", "web01.example.org"], ["url", "http://example.com/foo"]], "extra": {"emptyList": [], "emptyMap": {}, "length": 10837790, "results": [1, 2, 3, 4, 5], "session": {"foo": "bar"}, "unauthorized": false, "url": "http://example.org/foo/bar/"}, "metadata": {"title": "This is an example Python exception", "in_app_frame_mix": "system-only"}, "fingerprint": ["{{ default }}"], "hashes": ["3a2b45089d0211943e5a6645fb4cea3f"], "culprit": "raven.scripts.runner in main", "title": "This is an example Python exception", "location": null, "_ref": 4506964610318336, "_ref_version": 2, "_metrics": {"bytes.stored.event": 8272}, "nodestore_insert": 1717749673.44952, "id": "a1fdd396763241eda55f7e42178c4ed4"}}
 */

async fn send_to_discord(event: sentry_definition::SentryEvent) {
    let discord_webhook_res = env::var("DISCORD_WEBHOOK");
    let discord_webhook = match discord_webhook_res {
        Ok(val) => val,
        Err(_e) => {
            return;//no discord webhook
        }
    };

    let mut map = serde_json::Map::new();
    map.insert("username".to_string(), serde_json::Value::String("Sentry".to_string()));
    map.insert("avatar_url".to_string(), serde_json::Value::String("https://sentry.io/_assets/branding/png/sentry-horizontal-black.png".to_string()));

    let mut embeds = Vec::new();
    let mut embed = serde_json::Map::new();
    embed.insert("title".to_string(), serde_json::Value::String(event.message.clone()));
    embed.insert("description".to_string(), serde_json::Value::String(event.culprit.clone()));
    embed.insert("url".to_string(), serde_json::Value::String(event.url.clone()));
    embed.insert("color".to_string(), serde_json::Value::Number(serde_json::Number::from(16711680)));
    // ISO 8601
    //    "timestamp": 1717749612.583,
    let timestamp = DateTime::<Utc>::from_timestamp_millis(event.event.timestamp as i64);
    if timestamp.is_none() {
        // use current time if we can't parse the timestamp from the
        embed.insert("timestamp".to_string(), serde_json::Value::String(Utc::now().to_rfc3339()));
    }else{
        let timestamp = timestamp.unwrap();
        embed.insert("timestamp".to_string(), serde_json::Value::String(timestamp.to_rfc3339()));
    }

    let mut fields = Vec::new();
    fields.push(serde_json::json!({
        "name": "Project",
        "value": event.project_name,
        "inline": true
    }));
    fields.push(serde_json::json!({
        "name": "Environment",
        "value": event.event.environment,
        "inline": true
    }));
    fields.push(serde_json::json!({
        "name": "Level",
        "value": event.event.level,
        "inline": true
    }));
    fields.push(serde_json::json!({
        "name": "Logger",
        "value": event.event.logger,
        "inline": true
    }));
    fields.push(serde_json::json!({
        "name": "Platform",
        "value": event.event.platform,
        "inline": true
    }));
    fields.push(serde_json::json!({
        "name": "Version",
        "value": event.event.version,
        "inline": true
    }));

    embed.insert("fields".to_string(), serde_json::Value::Array(fields));
    embeds.push(serde_json::Value::Object(embed));
    map.insert("embeds".to_string(), serde_json::Value::Array(embeds));


    let client = reqwest::Client::new();
    let res = client.post(discord_webhook)
        .json(&map)
        .send()
        .await;
    match res {
        Ok(_) => println!("Sent to Discord"),
        Err(e) => println!("Error sending to Discord: {}", e)
    }
}

async fn handle_sentry(item: web::Json<sentry_definition::SentryEvent>) -> impl Responder {
    println!("Received Sentry event: {:?}", item);
    send_to_discord(item.clone()).await;

    "OK"
}


#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    // check Discord webhook
    let discord_webhook_res = env::var("DISCORD_WEBHOOK");
    match discord_webhook_res {

        Ok(val) => println!("Discord webhook found {}", val),
        Err(e) => println!("No Discord webhook found {}", e)
    }
    let port = env::var("PORT").unwrap_or("8080".to_string());
    let port_int = port.parse::<u16>().unwrap();
    println!("Starting server on port {}", port_int);
    println!("URL: http://localhost:{}/sentry", port_int);
    HttpServer::new(|| {
        App::new()
        .wrap(middleware::Logger::default())
        .app_data(web::JsonConfig::default().limit(1024 * 32)) // <- limit size of the payload (global configuration)
        .service(web::resource("/sentry").route(web::post().to(handle_sentry)))
    })
    .bind(("0.0.0.0", port_int))?
    .run()
    .await
}