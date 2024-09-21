// use reqwest::get;
// use std::error::Error;
// use std::io::Cursor;
// use image::DynamicImage;
// //use image::ImageReader;

// // Fetch the image from the URL
// pub async fn fetch_image(url: &str) -> Result<DynamicImage, Box<dyn Error>> {
//     let response = get(url).await?;
//     let bytes = response.bytes().await?.to_vec();
    
//     let img = ImageReader::new(Cursor::new(bytes))
//         .with_guessed_format()?
//         .decode()?;
    
//     Ok(img)
// }

// // Display the image as ASCII art in the terminal
// pub async fn image_to_ascii(url: &str) -> Result<(), Box<dyn Error>> {
//      // Fetch the image from the URL
//      let response = get(url).await?;
//      let bytes = response.bytes().await?.to_vec();
     
//      // Load the image from the bytes
//      //let img = Image::from_dynamic(Cursor::new(bytes))?;
     
//      // Convert image to ASCII and print it
//      //println!("{}", img.to_string());
     
//      Ok(())
// }
