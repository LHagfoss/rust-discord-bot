use crate::{Context, Error};
use chrono::Utc;
use chrono_tz::Tz;

#[poise::command(slash_command, prefix_command)]
pub async fn time(
    ctx: Context<'_>,
    #[description = "The city or country to get the time for (e.g., 'Oslo', 'Norway')"]
    location: String,
) -> Result<(), Error> {
    let location_str = location.replace(" ", "_");

    let timezone: Result<Tz, _> = location_str.parse();

    match timezone {
        Ok(tz) => {
            let now = Utc::now().with_timezone(&tz);
            let time_str = now.format("%H:%M:%S").to_string();
            let response = format!("The current time in {} is {}.", location, time_str);
            ctx.say(response).await?;
        }
        Err(_) => {
            let response = format!("Could not find a timezone for '{}'. Please use a valid IANA timezone name (e.g., 'Europe/Oslo', 'America/New_York'). You can find a list of valid timezones here: https://en.wikipedia.org/wiki/List_of_tz_database_time_zones", location);
            ctx.say(response).await?;
        }
    }

    Ok(())
}