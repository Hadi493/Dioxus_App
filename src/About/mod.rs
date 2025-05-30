use dioxus::prelude::*;


pub fn About() -> Element {
   rsx! {
       section { 
           class: "container",

           h1 {
               class: "head",
               "Hello About"
           }
       }
   }    
}

