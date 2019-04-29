/*
actix::run(|| {
    client::ClientRequest::get("http://www.rust-lang.org") // <- Create request builder
        .header("User-Agent", "Actix-web")
        .finish().unwrap()
        .send()                                    // <- Send http request
        .map_err(|_| ())
        .and_then(|response| {                     // <- server http response
            println!("Response: {:?}", response);
            actix::System::current().stop();
            Ok(())
        })
});
*/