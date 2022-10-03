use crate::components::{ExperienceContainer, GeneralContainer, Icon, ListItem, TableItem, Type};
use js_sys::Date;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {}

pub struct About {
  age: u32,
}

impl Component for About {
  type Message = ();
  type Properties = Props;

  fn create(_ctx: &Context<Self>) -> Self {
    let dob = Date::parse("19 May 2000");
    let difference = Date::now() - dob;
    let age = (difference / 3.154e+10) as u32;

    Self { age }
  }

  fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
    false
  }

  fn view(&self, _ctx: &Context<Self>) -> Html {
    let social_classes = classes!(
      "bg-base-lt hover:bg-tertiary-accent-md rounded-full mx-4 transition-colors".to_string()
    );

    html! {
      <div class="flex flex-col justify-center bg-base-dk bg-opacity-80 rounded-2xl items-center overflow-x-hidden">
        <div class="py-2 animate-blur-in">
          <div class="flex justify-center items-center mb-4">
            <a class={social_classes.clone()} href="https://github.com/kareigu" target="_blank">
              <Icon icon="gh" />
            </a>
            <a class={social_classes.clone()} href="https://twitter.com/kareigu" target="_blank">
              <Icon icon="twitter" disable_invert={true} />
            </a>
            <a class={social_classes.clone()} href="https://steamcommunity.com/id/kareigu/" target="_blank">
              <Icon icon="steam" disable_invert={true} />
            </a>
          </div>
          <div class="text-center mb-4 mx-2">
            <p>{ format!("{} year old guy from Southern Finland with an interest in technology and creating things.", self.age) }</p>
            <p class="mt-4">{ "Currently studying Business Information Technology at the Haaga-Helia University of Applied Sciences, focusing on
            software development." }</p>
            <p class="mt-4">{ "On the hobby side I enjoy programming, photography, video editing, 3D-modeling, playing guitar and writing music" }</p>
          </div>
        </div>

        <div class="flex flex-col desktop:flex-row desktop:items-start justify-center items-center">
          <div class="flex flex-col justify-center items-center">
            <h1 class="font-mulish text-4xl">{ "Experience" }</h1>
            <ExperienceContainer
              img="/static/abb.png"
              title="IT Trainee"
              location="Helsinki, Finland"
              company="ABB Drives"
              timespan="May 2021 - Present"
            >
            <>
              <ListItem text="Working full time during summer, part time rest of the year" />
              <ListItem text="Updating and working on a legacy document management software (C++, Qt)" />
              <ListItem text="Creating a program for automating AR-workspace cleanup (TypeScript)" />
              <ListItem text="Working on an augmented reality application (JavaScript, Angular.js)" />
              <ListItem text="Producing instructional videos and documentation on AR/VR applications" />
              <ListItem text="Worked on creating an algorithm for automating manual generation (TypeScript)" />
              <ListItem text="Creating animated service manual instructions from 3D-models" />
              <ListItem text="General documentation and DevOps" />
            </>
            </ExperienceContainer>
            <ExperienceContainer
              img="/static/fdf.png"
              title="Virtual Training Assistant"
              location="Vekaranjärvi, Kouvola, Finland"
              company="Finnish Defence Forces"
              timespan="February - September 2020"
            >
            <>
              <ListItem text="Development of a classroom management system using React and Go" />
              <ListItem text="Further development of various other web services (VueJS, express.js, MongoDB)" />
              <ListItem text="Unreal Engine 4 powered VR-simulator development" />
              <ListItem text="Creating a virtualised environment for Virtual Battlespace 3" />
              <ListItem text="Virtual Battlespace 3 addon development" />
              <ListItem text="Editing, production and recording of a domestic recruitment video for new conscripts" />
              <ListItem text="Editing, production and recording of an international VBS3 addon demonstration video" />
              <ListItem text="Systems administration work (Ubuntu, Windows Server, Docker etc.)" />
              <ListItem text="Installing server and desktop hardware" />
              <ListItem text="Running networking and power extensions to a classroom" />
              <ListItem text="Technical support during simulator training sessions" />
              <ListItem text="Worked in a group to evaluate entrance exams and train new conscripts for this position" />
              <ListItem text="Teaching Unreal Engine 4 basics domestically to new conscripts" />
            </>
            </ExperienceContainer>
            <ExperienceContainer
              img="/static/3step.png"
              title="Work experience program"
              location="Helsinki, Finland"
              company="3Step IT Oy"
              timespan="1 week in 2014"
            >
            <>
              <ListItem text="Reinstalling laptops" />
              <ListItem text="QA-testing on a new internal service" />
            </>
            </ExperienceContainer>
          </div>

          <div class="flex flex-col justify-center items-center">
            <h1 class="font-mulish text-4xl">{ "Studies" }</h1>
            <ExperienceContainer
              img="/static/hh.png"
              title="BBA - Business Information Technology"
              location="Pasila, Helsinki, Finland"
              company="Haaga-Helia University of Applied Sciences"
              timespan="August 2019 - Present"
            >
            <>
              <ListItem text="Majoring in software development" />
            </>
            </ExperienceContainer>

            <GeneralContainer title="Languages" content_type={Type::Table}>
              <TableItem label="🇫🇮 Finnish:" text="🔵🔵🔵🔵🔵" />
              <TableItem label="🇬🇧 English:" text="🔵🔵🔵🔵🔵" />
              <TableItem label="🇸🇪 Swedish:" text="🔵🔵🔵⚫⚫" />
              <TableItem label="🇯🇵 Japanese:" text="🔵⚫⚫⚫⚫" />
              <TableItem label="🇩🇪 German:" text="🔵⚫⚫⚫⚫" />
            </GeneralContainer>

            <GeneralContainer title="Skills" content_type={Type::List}>
              <li class="text-center">
                <dl>
                  <dl>{"📝 Programming languages:"}</dl>
                  <dd>{"JavaScript/TypeScript"}</dd>
                  <dd>{"C++"}</dd>
                  <dd>{"Rust"}</dd>
                  <dd>{"Go"}</dd>
                </dl>
              </li>

              <li class="text-center mt-4">
                <dl>
                  <dl>{"🎨 Frameworks:"}</dl>
                  <dd>{"React"}</dd>
                  <dd>{"Angular"}</dd>
                  <dd>{"express.js"}</dd>
                  <dd>{"Svelte"}</dd>
                  <dd>{"actix-web"}</dd>
                  <dd>{"Rocket.rs"}</dd>
                  <dd>{"Yew.rs"}</dd>
                  <dd>{"Fiber"}</dd>
                  <dd>{"Qt"}</dd>
                </dl>
              </li>

              <li class="text-center mt-4">
                <dl>
                  <dl>{"💾 Development software:"}</dl>
                  <dd>{"Visual Studio Code"}</dd>
                  <dd>{"vim"}</dd>
                  <dd>{"cmake"}</dd>
                  <dd>{"npm & yarn"}</dd>
                  <dd>{"Unreal Engine 4"}</dd>
                  <dd>{"MongoDB"}</dd>
                  <dd>{"Dgraph"}</dd>
                </dl>
              </li>

              <li class="text-center mt-4">
                <dl>
                  <dl>{"🖥 Systems:"}</dl>
                  <dd>{"Linux (Ubuntu, Arch Linux)"}</dd>
                  <dd>{"Windows Server 2016"}</dd>
                  <dd>{"Docker"}</dd>
                  <dd>{"nginx"}</dd>
                </dl>
              </li>

              <li class="text-center mt-4">
                <dl>
                  <dl>{"💿 Other software:"}</dl>
                  <dd>{"Adobe Photoshop"}</dd>
                  <dd>{"Adobe Lightroom"}</dd>
                  <dd>{"Adobe Premiere"}</dd>
                  <dd>{"Blender"}</dd>
                  <dd>{"DaVinci Resolve"}</dd>
                  <dd>{"Cubase"}</dd>
                </dl>
              </li>
            </GeneralContainer>
          </div>
        </div>

        <div class="flex flex-col justify-center items-center mb-6 animate-blur-in">
          <h2 class="font-mulish text-2xl">{ "Contact" }</h2>
          <p>{ "mail@karei.dev" }</p>

          <a class="hover:filter hover:invert" href="https://www.linkedin.com/in/eetu-kivel%C3%A4-6b2a66204/" target="_blank">
            <Icon icon="linked-in" />
          </a>
        </div>

      </div>
    }
  }
}
