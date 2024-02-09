use chrono::DateTime;
use leptos::{
    component, create_signal, leptos_dom::logging::console_log, set_interval, view, IntoView, SignalGet, SignalSet
};
use std::time::Duration;

use wasm_bindgen::prelude::wasm_bindgen;
use leptos::spawn_local;

use crate::progress;
struct Event {
    title: String,
    start: chrono::DateTime<chrono::Utc>,
    location: String,
    description: String,
    frequency: String,
}



#[wasm_bindgen]
pub async fn memes() -> String {
    let mut vec = vec![Event {
        title: String::new(),
        start: DateTime::parse_from_rfc3339("2021-01-01T00:00:00Z")
            .unwrap()
            .into(),
        location: String::new(),
        description: String::new(),
        frequency: String::new(),
    }];


    let current_semester = progress::get_current_semester().await;
    let current_semester = current_semester.as_string().unwrap();
    let current_semester: progress::Semester = progress::Semester {
        start: chrono::NaiveDate::parse_from_str(
            current_semester.split("&&").collect::<Vec<_>>()[1],
            "%Y-%m-%d",
        )
        .unwrap(),
        end: chrono::NaiveDate::parse_from_str(
            current_semester.split("&&").collect::<Vec<_>>()[2],
            "%Y-%m-%d",
        )
        .unwrap(),
        name: current_semester.split("&&").collect::<Vec<_>>()[0].to_string(),
    };

    let timestamp = current_semester.start.and_hms_opt(0, 0, 0).unwrap().timestamp();




    let url = format!("https://nextcloud.inphima.de/remote.php/dav/public-calendars/CAx5MEp7cGrQ6cEe?start={}&export=&componentType=VEVENT", timestamp);


    let resp = reqwest::get(url).await.unwrap();
    for i in resp.text().await.unwrap().split("UID:").collect::<Vec<_>>() {

        let i = i.replace('\\', "");
        
        let now = chrono::Utc::now().timestamp();


        if vec.len() > 7 {
            break;
        }
        let mut event = Event {
            title: String::new(),
            location: String::new(),
            start: DateTime::parse_from_rfc3339("2021-01-01T00:00:00Z")
                .unwrap()
                .into(),
            description: String::new(),
            frequency: String::new(),
        };

        if i.contains("SUMMARY:") {
            event.title = i.split("SUMMARY:").collect::<Vec<_>>()[1]
                .split('\n')
                .collect::<Vec<_>>()[0]
                .to_string();

        }

        if i.contains("DTSTART;TZID=Europe/Berlin:") {

            let date = i.split("DTSTART;TZID=Europe/Berlin:").collect::<Vec<_>>()[1]
                .split('\n')
                .collect::<Vec<_>>()[0]
                .to_string();

            //parse Date to DateTime 20230101T000000
            let date = format!("{}T{}Z", &date[0..8], &date[9..15]);
            let date = format!(
                "{}{}{}{}{}{}{}",
                &date[0..4],
                "-",
                &date[4..6],
                "-",
                &date[6..8],
                "T",
                &date[9..15]
            );
            let date = format!(
                "{}{}{}{}{}{}{}",
                &date[0..11],
                &date[11..13],
                ":",
                &date[13..15],
                ":",
                "00",
                "Z"
            );



            //check if date is in the past

            event.start = DateTime::parse_from_rfc3339(&date).unwrap().into();

        }

        event.location = "TBA".to_string();

        if i.contains("LOCATION:") {
            event.location = i.split("LOCATION:").collect::<Vec<_>>()[1]
                .split('\n')
                .collect::<Vec<_>>()[0]
                .to_string()
                .split('|')
                .collect::<Vec<_>>()[0]
                .to_string();

        }

        if i.contains("DESCRIPTION:") {
            event.description = i.split("DESCRIPTION:").collect::<Vec<_>>()[1]
                .split('\n')
                .collect::<Vec<_>>()[0]
                .to_string();

        }

        if i.contains("RRULE:FREQ=") {
            event.frequency = i.split("RRULE:FREQ=").collect::<Vec<_>>()[1]
                .split(';')
                .collect::<Vec<_>>()[0]
                .to_string();
        }


        //get next date if event is recurring
        if event.frequency == "WEEKLY" {
            let mut date = event.start;
            while date.timestamp() < now {
                date += chrono::Duration::weeks(1);
            }
            event.start = date;
        }

        if event.frequency == "MONTHLY" {
            let mut date = event.start;
            while date.timestamp() < now {
                date += chrono::Duration::days(30);
            }
            event.start = date;
        }

        if event.frequency == "YEARLY" {
            let mut date = event.start;
            while date.timestamp() < now {
                date += chrono::Duration::days(365);
            }
            event.start = date;
        }

        if event.frequency == "DAILY" {
            let mut date = event.start;
            while date.timestamp() < now {
                date += chrono::Duration::days(1);
            }
            event.start = date;
        }

        if event.start.timestamp() < now {
            continue;
        }


        if !event.title.is_empty() {
            vec.push(event);
        }
    }

    //sort after date

    vec.sort_by(|a, b| a.start.cmp(&b.start));

    //format Date to string

    let mut string = String::new();

    string = string
        + &vec[1].title
        + " && "
        + &vec[1].start.format("%d.%m.%Y %H:%M").to_string()
        + " && "
        + &vec[1].location
        + " && "
        + &vec[1].description
        + "\n";


    for i in 2..vec.len() {
        if vec[i].title != vec[i - 1].title {
            string = string
                + &vec[i].title
                + " && "
                + &vec[i].start.format("%d.%m.%Y %H:%M").to_string()
                + " && "
                + &vec[i].location
                + " && "
                + &vec[i].description
                + "\n";
        }
    }

    console_log(&string);

    string
}

#[component]
pub fn App() -> impl IntoView {
    let (events, set_events) = create_signal(vec![vec![String::new()]]);
    spawn_local(async move {
        let events = memes().await;

        let mut tmp = vec![vec![String::new()]];


        for i in events.split('\n').collect::<Vec<_>>() {
            tmp.push(i.split(" && ").map(|x| x.to_string()).collect::<Vec<_>>());
        }

        set_events.set(tmp);


    });

    set_interval(
        move || {
            spawn_local(async move {
                let events = memes().await;

                let mut tmp = vec![vec![String::new()]];

                for i in events.split('\n').collect::<Vec<_>>() {
                    tmp.push(i.split(" && ").map(|x| x.to_string()).collect::<Vec<_>>());
                }

                set_events.set(tmp);


            });
        },
        Duration::from_secs(60 * 30),
    );

    view! {

        <div style="width:100%; height:100%">
            <ul style="list-style-type:none;padding-left:0px">
            {move || events.get().iter().map(move |x| {
              if x[0].is_empty() {
                  view! {
                      <li class="hidden" style="width:100%">

                          </li>
                          <li>
                          </li>
                  }
              }else{
                  if x[2].len() > 17 {
                      return view! {
                          <li style="width:100%; font-size:1.8vw; color: #00cc00; padding-bottom:0px">
                          {x[1].clone()}
                          </li>
                              <li style="width:100%; font-size:1.8vw;padding-bottom:10px; white-space:initial">

                              {x[0].clone()}
                          </li><li style="padding-bottom:30px; font-size:1.3vw">
                              siehe Kalender
                              </li>

                      };
                  }
                  view! {
                      <li style="width:100%; font-size:1.8vw; color: #00cc00; padding-bottom:0px">
                      {x[1].clone()}
                      </li>
                          <li style="width:100%; font-size:1.8vw;padding-bottom:10px; white-space:initial">

                          {x[0].clone()}
                      </li><li style="padding-bottom:30px; font-size:1.3vw">
                      {x[2].clone()}
                      </li>

                  }
              }

          }
            ).collect::<Vec<_>>()}
        </ul>
            </div>

    }
}
