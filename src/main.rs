use clap::Parser;
use opencv::{
    highgui, 
    imgcodecs::{imencode},
    prelude::*, 
    videoio, 
    core::Vector,
    Result
};
use base64::prelude::*;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// RTSP feed url
    #[arg(short, long)]
    source: String,
}

fn main() -> Result<()> {

    let debug = 0;

/*
    let args: Vec<String> = env::args().collect();
    let args = Args::parse();
    let query = &args[1];
    let video_source = &args[2];

    println!("Searching for {query}");
    println!("Video source {file_path}");
*/

/*
    let matches = Command::new("cvaas")
        .version("1.0")
        .about("CVaaS Inferencing")
        .arg(arg!(--source <VALUE>).required(true))
        .get_matches();

    println!(
        "source: {:?}",
        matches.get_one::<String>("source").expect("required")
    );
*/

/*
    // Set some immutable variables
    // Triton Server
    let triton_server_ip    = "localhost";
    let triton_server_port  = 8001;
    let triton_server_url   = format!("http://{}:{}", triton_server_ip, triton_server_port);

    // MQTT broker
    let mqtt_broker_ip      = "localhost";
    let mqtt_broker_port    = 1883;
    let mqtt_broker_url     = format!("mqtt://{}:{}", mqtt_broker_ip, mqtt_broker_port);
    let mqtt_broker_user    = "";

    // MQTT channels
    //mqtt_topic_raw          = "gg/video/raw/" + location_code + "/" + stream
    //mqtt_topic_infer        = "gg/video/inf/" + location_code + "/" + stream
    //mqtt_topic_infer_boxes  = "gg/boxes/inf/" + location_code + "/" + stream

    // Redis
    let redis_host = "";
    let redis_port = "";
*/

/*
    // https://github.com/octoml/triton-client-rs
    // un-auth'd use of Triton
    let client = Client::new("http://localhost:8001/", None).await?;
    let models = client
        .repository_index(triton_client::inference::RepositoryIndexRequest {
            repository_name: "".into(), // This should show us models not referenced by repo name.
            ready: false,               // show all models, not just ready ones.
        })
        .await?;

    // Triton Client
    // https://github.com/octoml/triton-client-rs/blob/main/examples/repo_index.rs
    // un-auth'd use of Triton
    //let client = Client::new("http://localhost:8001/", None).await?;
    let client = Client::new(triton_server_url, None).await;
    let models = client.expect("REASON").repository_index(triton_client::inference::RepositoryIndexRequest {
            repository_name: "".into(), // This should show us models not referenced by repo name.
            ready: false,               // show all models, not just ready ones.
        })
        .await;

    println!("Running models:");

    for model in models.unwrap().models.iter() {
        println!("    {:?}", model);
    }

    category_index = label_map_util.create_category_index_from_labelmap("./labels.txt", use_display_name=True)
    if debug == 1:
        println!("CATEGORY INDEX");
        print(category_index)

    // Setup MQTT client
    let host = env::args()
        .nth(1)
        .unwrap_or_else(|| "mqtt://localhost:1883".to_string());

    println!("Connecting to the MQTT server at '{}'", host);

    // Create the client
    let cli = mqtt::AsyncClient::new(host).unwrap_or_else(|err| {
        println!("Error creating the client: {}", err);
        process::exit(1);
    });

     if let Err(err) = block_on(async {
        // Connect with default options and wait for it to complete or fail
        // The default is an MQTT v3.x connection.
        cli.connect(None).await?;

        // Create a message and publish it
        println!("Publishing a message on the topic 'test'");
        let msg = mqtt::Message::new("test", "Hello Rust MQTT world!", mqtt::QOS_1);
        cli.publish(msg).await?;

        // Disconnect from the broker
        println!("Disconnecting");
        cli.disconnect(None).await?;

        Ok::<(), mqtt::Error>(())
    }) {
        eprintln!("{}", err);
    }
*/

    // let rtsp_url = "rtsp://1701954d6d07.entrypoint.cloud.wowza.com:1935/app-m75436g0/27122ffc_stream2";
    // let rtsp_url = "http://107.0.231.40:8083/view/index.shtml";
    // let rtsp_url = "rtsp://freja.hiof.no:1935/rtplive/definst/hessdalen03.stream";
    let rtsp_url = "./src/intersection-night.mp4";

    let window = "video capture";
    highgui::named_window(window, highgui::WINDOW_AUTOSIZE)?;

    let mut cap = videoio::VideoCapture::from_file(rtsp_url, videoio::CAP_ANY)?; // 0 is the default camera
    let opened = videoio::VideoCapture::is_opened(&cap)?;
    if !opened {
        panic!("Unable to open RTSP feed!");
    }

    loop {
        let mut frame = Mat::default();
        cap.read(&mut frame)?;

        // Encode frame
        let mut buffer = Vector::<u8>::new();
        imencode(".jpg", &frame, &mut buffer, &opencv::core::Vector::<i32>::new()).unwrap();

        // Now `buffer` contains the encoded image data as a jpeg
        if debug > 0 {
            println!("Encoded image size: {} bytes", buffer.len());
        }

        // Encode jpeg data to text for MQTT
        let encoded = BASE64_STANDARD.encode(buffer);
        if debug > 0 {
            println!("Encoded image: {} ", encoded);
        }

        if frame.size()?.width > 0 {
            highgui::imshow(window, &frame)?;
        }

        let gui_wait = 30;
        let key = highgui::wait_key(gui_wait)?;
        if key > 0 && key != 255 {
            break;
        }
    }
    cap.release()?;
    Ok(())
}
