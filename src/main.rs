

//use imageHandler::image_to_ascii;
use search::{get_album_details, get_band_details, search_by_album_title, search_by_band_name, search_by_genre, Band, FullAlbum};
use std::{error::Error, future::Future, io::{self, Write}, pin::Pin};
mod cli;
mod search;
mod imageHandler;

fn print_banner() {
    println!(
        r#"
         _____                       _                            _ _       
        | ____|_ __   ___ _   _  ___| | ___  _ __   __ _  ___  __| (_) __ _ 
 _____  |  _| | '_ \ / __| | | |/ __| |/ _ \| '_ \ / _` |/ _ \/ _` | |/ _` |
|_____| | |___| | | | (__| |_| | (__| | (_) | |_) | (_| |  __/ (_| | | (_| |
 __  __ |_____|_| |_|\___|\__, |\___|_|\___/| .__/ \__,_|\___|\__,_|_|\__,_|
|  \/  | ___| |_ __ _| | ||___/ _ __ ___    |_|                             
| |\/| |/ _ \ __/ _` | | | | | | '_ ` _ \   _____                           
| |  | |  __/ || (_| | | | |_| | | | | | | |_____|                          
|_|  |_|\___|\__\__,_|_|_|\__,_|_| |_| |_|                                  

                                                                                                                                                                                         
        "#
    );
}

#[derive(Debug)]
pub enum SearchResult {
    Band(Vec<Band>),
    Album(Vec<FullAlbum>),
}


type SearchFn = for<'a> fn(&'a str) -> Pin<Box<dyn Future<Output = Result<SearchResult, reqwest::Error>> + Send + 'a>>;

fn boxed_search_by_band_name<'a>(query: &'a str) -> Pin<Box<dyn Future<Output = Result<SearchResult, reqwest::Error>> + Send + 'a>> {
    Box::pin(async move {
        let result = search_by_band_name(query).await?;
        Ok(SearchResult::Band(result))
    })
}

fn boxed_search_by_genre<'a>(query: &'a str) -> Pin<Box<dyn Future<Output = Result<SearchResult, reqwest::Error>> + Send + 'a>> {
    Box::pin(async move {
        let result = search_by_genre(query).await?;
        Ok(SearchResult::Band(result))
    })
}

fn boxed_search_by_album_title<'a>(query: &'a str) -> Pin<Box<dyn Future<Output = Result<SearchResult, reqwest::Error>> + Send + 'a>> {
    Box::pin(async move {
        let result = search_by_album_title(query).await?;
        Ok(SearchResult::Album(result))
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    print_banner();

    println!("Choose a search option:");
    println!("1. Search by band name");
    println!("2. Search by genre");
    println!("3. Search by album title");

    let mut choice = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut choice).expect("Failed to read input");
    let choice: usize = choice.trim().parse().expect("Please enter a valid number");

    let (search_label, search_fn): (&str, SearchFn) = match choice {
        1 => ("band name", boxed_search_by_band_name),
        2 => ("genre", boxed_search_by_genre),
        3 => ("album title", boxed_search_by_album_title),
        _ => {
            println!("Invalid choice.");
            return Ok(());
        }
    };

    // Get query input from the user
   let mut query = String::new();
    print!("Enter {}: ", search_label);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut query).expect("Failed to read input");
    query = query.trim().to_string();


    match search_fn(&query).await {
        Ok(result) => {
            match result {
                SearchResult::Band(bands) => {
                    if bands.is_empty() {
                        println!("No results found for '{}'", query);
                    } else {
                        println!("Found the following results for '{}':", query);
                        for (index, band) in bands.iter().enumerate() {
                            println!("{}. Band: {}, Genre: {}, Country: {}", index + 1, band.name, band.genre, band.country);
                        }
    
                        // Select a band for detailed info
                        let mut selection = String::new();
                        println!("Enter the number of the band you want more details for: ");
                        io::stdout().flush().unwrap();
                        io::stdin().read_line(&mut selection).expect("Failed to read input");
                        let selection: usize = selection.trim().parse().expect("Please enter a valid number");
    
                        if selection > 0 && selection <= bands.len() {
                            let selected_band = &bands[selection - 1];
                            match get_band_details(&selected_band.id).await {
                                Ok(band_details) => {
                                    println!("\nDetails about '{}':", band_details.name);
                                    println!("Genre: {}", band_details.genre);
                                    println!("Country: {}", band_details.country);
                                    if let Some(formed_in) = &band_details.formedIn {
                                        println!("Formed in: {}", formed_in);
                                    }
                                    if let Some(years_active) = &band_details.yearsActive {
                                        println!("Years Active: {}", years_active);
                                    }
                                    if let Some(location) = &band_details.location {
                                        println!("Location: {}", location);
                                    }
                                    if let Some(themes) = &band_details.themes {
                                        println!("Themes: {}", themes);
                                    }
                                    if let Some(label) = &band_details.label {
                                        println!("Label: {}", label);
                                    }
                                    if let Some(albums) = &band_details.albums {
                                        println!("\nAlbums:");
                                        for album in albums {
                                            println!("Name: {}, Type: {}, Date: {}, Link: {}", album.name, album.type_.as_deref().unwrap_or("N/A"), album.year.as_deref().unwrap_or("N/A"), album.link);
                                        }
                                    }
                                }
                                Err(e) => eprintln!("Error occurred while fetching band details: {}", e),
                            }
                        } else {
                            println!("Invalid selection.");
                        }
                    }
                }
                SearchResult::Album(albums) => {
                    if albums.is_empty() {
                        println!("No results found for '{}'", query);
                    } else {
                        println!("Found the following results for '{}':", query);
                        for (index, album) in albums.iter().enumerate() {
                            println!("{}. Album: {}, Type: {}, Date: {}", index + 1, album.title, album.type_.as_deref().unwrap_or("N/A"), album.date.as_deref().unwrap_or("N/A"));
                        }
    
                        // Select a band for detailed info
                        let mut selection = String::new();
                        println!("Enter the number of the band you want more details for: ");
                        io::stdout().flush().unwrap();
                        io::stdin().read_line(&mut selection).expect("Failed to read input");
                        let selection: usize = selection.trim().parse().expect("Please enter a valid number");
    
                        if selection > 0 && selection <= albums.len() {
                            let selected_album = &albums[selection - 1];
                            match get_album_details(&selected_album.id).await {
                                Ok(album_details) => {
                                    println!("\nDetails about '{}':", album_details.name);
                                    println!("Album type: {}", album_details.album_type);
                                    println!("Release date: {}", album_details.releaseDate);
                                    if let Some(label) = &album_details.label {
                                        println!("Label: {}", label);
                                    }
                                    if let Some(albumFormat) = &album_details.albumFormat {
                                        println!("Format: {}", albumFormat);
                                    }
                                    if let Some(songs) = &album_details.songs {
                                        println!("\nSongs:");
                                        for song in songs {
                                            println!("\nNumber: {} Name: {} Lenght: {} Lyrics: {}", song.number, song.name, song.length, song.lyrics);
                                        }
                                    }
                                }
                                Err(e) => eprintln!("Error occurred while fetching band details: {}", e),
                            }
                        } else {
                            println!("Invalid selection.");
                        }
                    }

                    // if albums.is_empty() {
                    //     println!("No albums found for '{}'", query);
                    // } else {
                    //     println!("Found the following albums for '{}':", query);
                    //     for (index, album) in albums.iter().enumerate() {
                    //         println!("{}. Album: {}, Type: {}, Date: {}", index + 1, album.title, album.type_.as_deref().unwrap_or("N/A"), album.date.as_deref().unwrap_or("N/A"));
                    //     }
                    // }
                }
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    Ok(())
}
