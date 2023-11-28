use std::time::Duration;
use leptos::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{RequestInit, RequestMode, Request, Response, RequestCache};
#[component]
pub fn App2() -> impl IntoView {
    view! {
        <Essen id="348".to_string()/> 
    }
}

#[component]
fn Essen(id:String) -> impl IntoView { 
    let (state,set_state) = create_signal(vec![vec![String::new()]]);

    let id2 = id.clone();

    spawn_local(async move {
        let list = get_menu(id2.clone());
        let list = list.await.unwrap().as_string().unwrap();
        set_state.set(list.split("\n").map(|x| x.split(" && ").map(|x| x.to_string()).collect::<Vec<_>>()).collect::<Vec<_>>());
    });

    set_interval(
        move || {
            let id = id.clone();
            spawn_local(async move {
                let list = get_menu(id);
                let list = list.await.unwrap().as_string().unwrap();
                set_state.set(list.split("\n").map(|x| x.split(" && ").map(|x| x.to_string()).collect::<Vec<_>>()).collect::<Vec<_>>());
            });
        }, Duration::from_secs(60*60*30),
    );

   
    view! {
        <table class="center" id="mensa" >
            <tr> 
            {move || state.get().iter().map(move |x| {
                 if x[0].is_empty() {
                     return view! {
                         <td class="hidden">
                         </td>
                     };
                 }else{
                     let style = format!("background-image: url({}); background-size: 110%; background-repeat: no-repeat; background-position: center; height: 100%; width: 100%; padding:0px", x[2].clone());
                         if x[3].clone() == "true" {
                             return view! {
                                 <td style=style>
                                     <div style="width:100%; height:auto; background:#3d3d3d; color:white;">
                                        <div style="width:calc(90% - 20px); background-color:#000000; color:#ffffff; margin:0px;overflow:hidden; text-overflow:ellipsis; height:fit-content;padding:10px">
                                            {x[1].clone()} </div>
                                        <div style="width:10%;padding-top:10px;padding-bottom:10px;color:white;">
                                            "V"
                                        </div>
                                    </div>
                                 </td>
                             }
                         }
                         return view! {
                             <td style=style> 
                                 <p style="background-color:#000000; color:#ffffff; margin:0px; width:calc(100% - 20px);overflow:hidden; text-overflow:ellipsis;padding:10px;">
                                    {x[1].clone()}
                                 </p>
                             </td>
                         }
                     }
                 }).collect::<Vec<_>>()
            } 
            </tr>
        </table>
    }
}

#[wasm_bindgen]
pub async fn get_food_pic(id:String) -> Result<JsValue, JsValue> {
    let mut today = chrono::Local::now().format("%d.%m.%Y").to_string();
    let time = chrono::Local::now().format("%H:%M").to_string();
    if chrono::Local::now().format("%u").to_string().parse::<i32>().unwrap() >= 5 {
        //set day to monday
        let diff_to_next_monday = 8 - chrono::Local::now().format("%u").to_string().parse::<i64>().unwrap();
        today = chrono::Local::now().checked_add_signed(chrono::Duration::days(diff_to_next_monday)).unwrap().format("%d.%m.%Y").to_string(); 
    }else if time > "14:30".to_string() {
        //set day to tomorrow
        today = chrono::Local::now().checked_add_signed(chrono::Duration::days(1)).unwrap().format("%d.%m.%Y").to_string(); 
    }

    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.cache(RequestCache::NoStore);
    opts.mode(RequestMode::Cors);
    let url = format!("https://www.stw-d.de/gastronomie/speiseplaene/essenausgabe-sued-duesseldorf/"); 
    let request = Request::new_with_str_and_init(&url, &opts)?;
    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    // `resp_value` is a `Reuponse` object.
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    // Convert this other `Promise` into a rust `Future`.
    let text = JsFuture::from(resp.text()?).await?.as_string().unwrap(); 
    let day = format!("data-date='{}'>", today);
    let day_info = text.split(&day);

    let essen = day_info.collect::<Vec<_>>()[1].split("</div>").collect::<Vec<_>>();

    for i in 0..essen.len() {
        if essen[i].contains(&id) {
            let url = essen[i].split("url(").collect::<Vec<_>>()[1].split(")").collect::<Vec<_>>()[0].replace("\"", "");
            return Ok(JsValue::from_str(&url));
        }
    }
    
    Ok(JsValue::from_str(&text))
}

#[wasm_bindgen]
pub async fn get_menu(id:String) ->Result<JsValue, JsValue> {
    let mut day = chrono::Local::now().format("%Y-%m-%d").to_string();
    let time = chrono::Local::now().format("%H:%M").to_string();
    if chrono::Local::now().format("%u").to_string().parse::<i32>().unwrap() >= 5 {
        //set day to monday
        let diff_to_next_monday = 8 - chrono::Local::now().format("%u").to_string().parse::<i64>().unwrap();
        day = chrono::Local::now().checked_add_signed(chrono::Duration::days(diff_to_next_monday)).unwrap().format("%Y-%m-%d").to_string(); 
    }else if time > "14:30".to_string() {
        //set day to tomorrow
        day = chrono::Local::now().checked_add_signed(chrono::Duration::days(1)).unwrap().format("%Y-%m-%d").to_string(); 
    }
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.cache(RequestCache::NoStore);
    opts.mode(RequestMode::Cors);
    let url = format!("https://openmensa.org/api/v2/canteens/{}/days/{}/meals", id, &day); 
    let request = Request::new_with_str_and_init(&url, &opts)?;
    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    // `resp_value` is a `Reuponse` object.
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    // Convert this other `Promise` into a rust `Future`.
    let text = JsFuture::from(resp.text()?).await?.as_string().unwrap(); 
    let mut essen = String::new();
    for i in 0..text.matches("name").count() {
    
        let essen_name = text.split("name\":").collect::<Vec<_>>()[i+1].split(",").collect::<Vec<_>>()[0].replace("\"", "");

        let essen_category = text.split("category\":").collect::<Vec<_>>()[i+1].split(",").collect::<Vec<_>>()[0].replace("\"", "");
        let is_vegan = text.split("notes\":").collect::<Vec<_>>()[i+1].contains("vegan"); 
        let pic_url = get_food_pic(essen_category.clone()).await.unwrap().as_string().unwrap();
        
        essen = format!("{} {} && {} && {} && {}\n", essen, essen_category, essen_name, pic_url, is_vegan);

    }

    Ok(JsValue::from_str(&essen))
}

