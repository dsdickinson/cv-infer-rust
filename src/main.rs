use clap::Parser;
use opencv::{
    highgui, 
    imgcodecs::{imencode, ImwriteFlags},
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

/*
#    let args: Vec<String> = env::args().collect();
#    let args = Args::parse();
#    let query = &args[1];
#    let video_source = &args[2];

#	println!("Searching for {query}");
#	println!("Video source {file_path}");
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
 * https://github.com/octoml/triton-client-rs
// un-auth'd use of Triton
let client = Client::new("http://localhost:8001/", None).await?;
let models = client
    .repository_index(triton_client::inference::RepositoryIndexRequest {
        repository_name: "".into(), // This should show us models not referenced by repo name.
        ready: false,               // show all models, not just ready ones.
    })
    .await?;
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
        println!("Encoded image size: {} bytes", buffer.len());

        // Encode jpeg data to text for MQTT
        let encoded = BASE64_STANDARD.encode(buffer);
        println!("Encoded image: {} ", encoded);

        if frame.size()?.width > 0 {
            highgui::imshow(window, &frame)?;
        }

        let key = highgui::wait_key(60)?;
        if key > 0 && key != 255 {
            break;
        }
    }
    cap.release()?;
    Ok(())
}
