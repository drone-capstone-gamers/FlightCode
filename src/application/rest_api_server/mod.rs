use std::sync::{Arc, Mutex};
use std::thread;
use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use actix_web::http::StatusCode;
use crate::application::data_manage::{DataSource, get_data_source_by_name, IncomingData};

async fn serve_frontend() -> HttpResponse {
    return HttpResponse::build(StatusCode::OK)
        .content_type("text/html")
        .body("<html><head><script src=\"https://ajax.googleapis.com/ajax/libs/jquery/3.7.1/jquery.min.js\"></script>\
        <link rel=\"stylesheet\" href=\"resource://content-accessible/plaintext.css\"></head>
                <body>
                <div id=\"battery_voltage\"></div>
                <script>


                    function get_data(source) {
                        let data_return = '';
                        $.get(source, function( data ) {
                            data_return = data;
                        });

                        return data_return;
                    }

                    $(document).ready(function() {
                    // Function to update element text
                    function updateBat() {
                      // Update the text of the element with the current timestamp
                      $('#battery_voltage').text(get_data('power').average_voltage);
                    }

                    // Call the updateText function every second (1000 milliseconds)
                    setInterval(updateBat, 1000);
                  });

                </script>
                </body>
                </html>");
}

async fn handle_get_request(current_data_storage: web::Data<Arc<Mutex<Box<[Option<IncomingData>; DataSource::COUNT]>>>>,
                            path: web::Path<(String,)>) -> HttpResponse {
    let source_string = path.0.clone();

    let source = get_data_source_by_name(source_string.clone());

    if source == DataSource::Invalid {
        return HttpResponse::build(StatusCode::BAD_REQUEST)
            .content_type("text")
            .body(format!("Invalid data source: {}!", source_string.clone()));
    }

    let current_data_result = &current_data_storage.lock().unwrap()[source as usize];

    if current_data_result.is_none() {
        return HttpResponse::build(StatusCode::NO_CONTENT)
            .content_type("text")
            .body(format!("There is no data yet for: {}", source_string.clone()));
    }

    let current_data = current_data_result.as_ref().unwrap();

    if current_data.file.is_some() {
        let response_data = current_data.file.as_ref().unwrap();
        return HttpResponse::build(StatusCode::OK)
            .content_type("image/png")
            .body(response_data.clone());
    }

    if current_data.serialized.is_some() {
        let current_data = current_data.serialized.as_ref().unwrap().to_string();
        return HttpResponse::build(StatusCode::OK)
            .content_type("json")
            .body(current_data);
    }

    return HttpResponse::build(StatusCode::NO_CONTENT)
        .content_type("text")
        .body(format!("Empty incoming data for: {}!", source_string.clone()));
}

pub fn spawn_rest_server(current_data_storage: Arc<Mutex<Box<[Option<IncomingData>; DataSource::COUNT]>>>) {
    thread::spawn(|| {
        let _ = run_rest_server(current_data_storage);
    });
}

#[actix_web::main]
async fn run_rest_server(current_data_storage: Arc<Mutex<Box<[Option<IncomingData>; DataSource::COUNT]>>>) -> std::io::Result<()> {
    return HttpServer::new(move || {
                App::new()
                    .app_data(web::Data::new(current_data_storage.clone()))
                    .route("/", web::get().to(serve_frontend))
                    .route("/{data_source}", web::get().to(handle_get_request))
            })
            .bind("0.0.0.0:8080").expect("Failed to bind address for REST API server!")
            .run()
            .await;
}
