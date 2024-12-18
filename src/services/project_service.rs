use crate::models::{LinkItem, Project, Technology};

pub fn get_projects() -> Vec<Project> {
    vec![
        // Workki AI
        Project {
            subtitle: "Workki AI".to_string(),
            title: "Web pages".to_string(),
            description: "To date, I have successfully added end-to-end tests with Cypress, significantly enhancing application reliability. I adapted the frontend to meet evolving business needs and migrated static data and blog content to the Sanity CMS, optimizing content management workflows. Additionally, I updated SEO practices to align with the latest standards, resulting in improved search engine visibility and traffic. I continue to contribute to ongoing development and optimization efforts.".to_string(),
            technologies: vec![VUE, NUXT, CYPRESS, SANITY],
            image_url: "assets/img/workki.PNG".to_string(),
            links: vec![
                LinkItem {
                    icon_url: Some("assets/icons/globe.svg".to_string()),
                    sr_text: Some("Workki AI website".to_string()),
                    url: "https://workkiai.com/".to_string()
                }
            ],
            reverse: true,
        },

         // Psyche's Royale Gaming ry (Altzone webpages)
        Project {
            subtitle: "Psyche's Royale Gaming ry / Volunteer work".to_string(),
            title: "Altzone webpages".to_string(),
            description: "I started these pages on my own, handling everything from layout and styling to decisions about the project's structure and design. As the project grew and the demands became more complex, I gradually began delegating responsibilities to new team members to manage the workload and maintain steady progress. Over time, I transitioned into the role of a technical lead, focusing on code reviews, ensuring the architecture remains scalable and maintainable, and supporting the team. I still occasionally implement key features, especially to set a standard or push the project forward.".to_string(),
            technologies: vec![FSD, REACT, NEXTJS, TS],
            // technologies: vec![FSD, REACT, NEXTJS, TS, JEST, RTL, STORYBOOK],
            image_url: "assets/img/altZone.PNG".to_string(),
            links: vec![
                LinkItem {
                    icon_url: Some("assets/icons/globe.svg".to_string()),
                    sr_text: Some("Altzone website".to_string()),
                    url: "https://altzone.fi".to_string()
                },

                LinkItem {
                    icon_url: Some("assets/icons/github.svg".to_string()),
                    sr_text: Some("Altzone-WebPages Github".to_string()),
                    url: "https://github.com/Alt-Org/Altzone-WebPages".to_string()
                },
            ],
            reverse: false,
        },


        // Thesis project (Device View Management System)
        Project {
            subtitle: "Thesis project (LAB, UAS)".to_string(),
            title: "Device View Management System".to_string(),
            description: "The thesis aimed to design and develop a web application dedicated to managing device content across various locations within LAB University of Applied Sciences, simplifying and streamlining the content update process. The project included a server-side data handling, a user-friendly admin panel, and a specialized view-app for each device.".to_string(),
            technologies: vec![FSD, REACT, REDUX, FORMIK],
            // technologies: vec![FSD, REACT, REDUX, FORMIK, TS, NODEJS, NEST, MONGODB, MONGOOSE, SOCKETIO],
            image_url: "assets/img/thesis.PNG".to_string(),
            links: vec![
                LinkItem {
                    icon_url: Some("assets/icons/book.svg".to_string()),
                    sr_text: Some("Device View Management system report".to_string()),
                    url: "https://www.theseus.fi/handle/10024/812819".to_string()
                }
            ],
            reverse: true,
        },
       
        // School investigation project (Task Manager)
        Project {
            subtitle: "School investigation project".to_string(),
            title: "Task Manager".to_string(),
            description: "A prototype user interface for a web app using a modular approach and modern technologies. Implemented Feature-Sliced Design, React, Redux Toolkit, Formik, Yup, TypeScript, and tested with Jest and React Testing Library. A storybook and Loki for visual regression testing were also integrated.".to_string(),
            technologies: vec![FSD, REACT, REDUX, TS, JEST, RTL],
            // technologies: vec![FSD, REACT, REDUX, TS, JEST, RTL, STORYBOOK, LOKI, EXPRESS],
            image_url: "assets/img/taskManager.PNG".to_string(),
            links: vec![
                LinkItem {
                    icon_url: Some("assets/icons/github.svg".to_string()),
                    sr_text: Some("Task Manager github".to_string()),
                    url: "https://github.com/leolabdev/task-manager-app".to_string()
                },
                LinkItem {
                    icon_url: Some("assets/icons/book.svg".to_string()),
                    sr_text: Some("Task Manager report".to_string()),
                    url: "https://drive.google.com/file/d/1aJC0YZ1vwFE6uvsGzY3KbRj0vRr2D9Fx/view?usp=sharing".to_string()
                }
            ],
            reverse: false,
        },
        // Solita dev-academy-2023-exercise (City Bike App)
        Project {
            subtitle: "Solita / dev-academy-2023-exercise".to_string(),
            title: "City Bike App".to_string(),
            description: "Developed a separated frontend and backend application. Backend as a REST API using Express and MariaDB, documented with Swagger, following MVC architecture. Frontend as a SPA using React and React-Bootstrap. The approach ensures maintainability and scalability.".to_string(),
            technologies: vec![TS, EXPRESS, MARIADB, ERD, REACT, BOOTSTRAP],
            image_url: "assets/img/cityBike.jpg".to_string(),
            links: vec![
                LinkItem {
                    icon_url: Some("assets/icons/github.svg".to_string()),
                    sr_text: Some("City Bike App Github".to_string()),
                    url: "https://github.com/leolabdev/dev-academy-2023-exercise".to_string()
                },
                LinkItem {
                    icon_url: Some("assets/icons/question.svg".to_string()),
                    sr_text: Some("City Bike App task description Github".to_string()),
                    url: "https://github.com/solita/dev-academy-2023-exercise".to_string()
                }
            ],
            reverse: true,
        },
        // Environmental sensing project competition(2022) - Sensors on wheels
        Project {
            subtitle: "Environmental sensing project competition(2022)".to_string(),
            title: "Sensors on wheels".to_string(),
            description: "An application for remotely controlling a robot, saving, and displaying collected environmental data. This was part of the Helsinki University environmental sensing project competition.".to_string(),
            technologies: vec![TS, Technology { name: "Canvas", url: "https://www.w3schools.com/html/html5_canvas.asp" }, REACT],
            image_url: "assets/img/robo.PNG".to_string(),
            links: vec![
                LinkItem {
                    icon_url: Some("assets/icons/github.svg".to_string()),
                    sr_text: Some("Sensors on wheels github".to_string()),
                    url: "https://github.com/MikhailDeriabin/RoboCar/tree/master/rApp".to_string()
                },
                LinkItem {
                    icon_url: Some("assets/icons/book.svg".to_string()),
                    sr_text: Some("Sensors on wheels Report".to_string()),
                    url: "https://drive.google.com/file/d/1yrqNzxcbNzylz2eGMhdFbAzaj6LNhVKX/view?usp=share_link".to_string()
                },
                LinkItem {
                    icon_url: Some("assets/icons/youtube.svg".to_string()),
                    sr_text: Some("Sensors on wheels youtube".to_string()),
                    url: "https://www.youtube.com/watch?v=5dwgvSX0VvQ".to_string()
                }
            ],
            reverse: false,
        },
        // Hobby Project (Smart App)
        Project {
            subtitle: "Hobby Project".to_string(),
            title: "Smart App".to_string(),
            description: "An application for managing smart devices. A hobby project with friends to practice skills. Implemented part of backend using NestJS, implemented MobX, and set up locales with i18n. Currently on hold.".to_string(),
            technologies: vec![TS, NEST, ERD, MOBX, I18N],
            image_url: "assets/img/smartApp.PNG".to_string(),
            links: vec![
                LinkItem {
                    icon_url: Some("assets/icons/github.svg".to_string()),
                    sr_text: Some("Smart App Github".to_string()),
                    url: "https://github.com/MikhailDeriabin/smart_app".to_string()
                },
                LinkItem {
                    icon_url: Some("assets/icons/book.svg".to_string()),
                    sr_text: Some("Smart App ERD".to_string()),
                    url: "https://drive.google.com/drive/folders/1HrT2jSdeSX4u8MU9tKYea3Ml6CRqdUKc?usp=share_link".to_string()
                }
            ],
            reverse: true,
        },
        // Hobby Project (Random API)
        Project {
            subtitle: "Hobby Project".to_string(),
            title: "Random API".to_string(),
            description: "Random API lets you add any kind of data and use it later as needed. Created the frontend using React and Bootstrap. Useful for generating random data (e.g. firstname-lastname pairs) easily.".to_string(),
            technologies: vec![Technology {name:"JavaScript", url:"https://www.javascript.com/"}, REACT, BOOTSTRAP],
            image_url: "assets/img/randomAPI.PNG".to_string(),
            links: vec![
                LinkItem {
                    icon_url: Some("assets/icons/github.svg".to_string()),
                    sr_text: Some("Random API Github".to_string()),
                    url: "https://github.com/leolabdev/random_api".to_string()
                },
                LinkItem {
                    icon_url: Some("assets/icons/youtube.svg".to_string()),
                    sr_text: Some("Random API Youtube".to_string()),
                    url: "https://www.youtube.com/watch?v=F9fZCMDTFhc".to_string()
                }
            ],
            reverse: false,
        },
        // LAB-Internship (Door Tablet App)
        Project {
            subtitle: "LAB-Internship".to_string(),
            title: "Door Tablet App".to_string(),
            description: "Application for class tablets at LAB UAS showing booking info. Recreated the app from scratch, updated dependencies, refactored logic into services, and added SSR support (Angular Universal), although later deemed unnecessary. Learned basics of Angular.".to_string(),
            technologies: vec![TS, ANGULAR, RXJS, FULLCALENDAR],
            image_url: "assets/img/doorApp.PNG".to_string(),
            links: vec![],
            reverse: true,
        },
        // Big-Flash - Internship (Map App)
        Project {
            subtitle: "Big-Flash - Internship".to_string(),
            title: "Map App".to_string(),
            description: "A tool to help delivery companies find the best route between places. First React project. Learned basics of React and React Leaflet. Realized importance of state management libraries after encountering props issues.".to_string(),
            technologies: vec![Technology {name:"JavaScript", url:"https://www.javascript.com/"}, REACT, RLEAFLET, CSS],
            image_url: "assets/img/mapApp2.PNG".to_string(),
            links: vec![
                LinkItem {
                    icon_url: Some("assets/icons/github.svg".to_string()),
                    sr_text: Some("Map App github".to_string()),
                    url: "https://github.com/leolabdev/map_app".to_string()
                },
                LinkItem {
                    icon_url: Some("assets/icons/book.svg".to_string()),
                    sr_text: Some("Map App Documentation".to_string()),
                    url: "https://drive.google.com/drive/folders/1hp9rjcgsTxuj1cKSFc4iTvVJtUcci6uY?usp=share_link".to_string()
                },
                LinkItem {
                    icon_url: Some("assets/icons/youtube.svg".to_string()),
                    sr_text: Some("Map App youtube".to_string()),
                    url: "https://www.youtube.com/watch?v=KZXvReT8jR4".to_string()
                }
            ],
            reverse: false,
        },
        // School Project (Students App)
        Project {
            subtitle: "School Project".to_string(),
            title: "Students App".to_string(),
            description: "App to handle students’ directory. Two APIs (api1 & api2) with students records in MongoDB. Api1 receives requests from frontend and forwards to Api2. Learned about Express and MongoDB structure.".to_string(),
            technologies: vec![TS, EXPRESS, Technology {name:"Handlebars", url:"https://handlebarsjs.com/"}, MONGODB, MONGOOSE],
            image_url: "assets/img/StudentsApp.PNG".to_string(),
            links: vec![
                LinkItem {
                    icon_url: Some("assets/icons/github.svg".to_string()),
                    sr_text: Some("The Students App github".to_string()),
                    url: "https://github.com/leolabdev/palvelintekniikat".to_string()
                }
            ],
            reverse: true,
        },
        // School Project (Converter)
        Project {
            subtitle: "School Project".to_string(),
            title: "Converter".to_string(),
            description: "A currency converter app created for a course. Built with Angular, TypeScript, and Angular Material. Learned basics of Angular and UI frameworks.".to_string(),
            technologies: vec![HTML5, CSS, TS, ANGULAR],
            image_url: "assets/img/Converter.PNG".to_string(),
            links: vec![
                LinkItem {
                    icon_url: Some("assets/icons/github.svg".to_string()),
                    sr_text: Some("The converter github".to_string()),
                    url: "https://github.com/leolabdev/Angular-Converter".to_string()
                }
            ],
            reverse: false,
        },
        // School Project (Terrain game)
        Project {
            subtitle: "School Project".to_string(),
            title: "Terrain game".to_string(),
            description: "A Unity project learning how to create terrain levels, add effects, and basic animations. Realized the fun and complexity of game development.".to_string(),
            technologies: vec![UNITY, CSHARP],
            image_url: "assets/img/terrainGame.PNG".to_string(),
            links: vec![
                LinkItem {
                    icon_url: Some("assets/icons/youtube.svg".to_string()),
                    sr_text: Some("Terrain game youtube link".to_string()),
                    url: "https://www.youtube.com/watch?v=mP_F4XZk7J4".to_string()
                }
            ],
            reverse: true,
        },
        // School Project (Weather station)
        Project {
            subtitle: "School Project".to_string(),
            title: "Weather station".to_string(),
            description: "Built a weather station to monitor atmosphere data. Sends data to the cloud via NodeMCU. Used Azure IoT Hub. Ideal for remote monitoring systems. Learned IoT basics.".to_string(),
            technologies: vec![ARDUINO, AZURE, FRITZING, DRAWIO],
            image_url: "assets/img/IoT.PNG".to_string(),
            links: vec![
                LinkItem {
                    icon_url: Some("assets/icons/book.svg".to_string()),
                    sr_text: Some("Weather station report".to_string()),
                    url: "https://docs.google.com/presentation/d/1oAuN59qByO3uJUjYJ906QYTqvUtq4hGJ/edit#slide=id.p1".to_string()
                }
            ],
            reverse: false,
        },
        // School Project (Weather station proto)
        Project {
            subtitle: "School Project".to_string(),
            title: "Weather station (proto)".to_string(),
            description: "Got introduced to IoT. Set up software, alert messages, connected sensors and created circuits. Used Raspberry Pi, Domoticz, and Prowl. This project served as a starting point for the later weather station project.".to_string(),
            technologies: vec![Technology {name:"Linux/Raspberry Pi", url:"https://www.raspberrypi.com/"}, DOMOTICZ, PROWL, FRITZING, DRAWIO],
            image_url: "assets/img/weatherSPrototype.PNG".to_string(),
            links: vec![
                LinkItem {
                    icon_url: Some("assets/icons/book.svg".to_string()),
                    sr_text: Some("Weather station(proto) report".to_string()),
                    url: "https://docs.google.com/presentation/d/11euympQB-LlnfVIRrhEVKFgXiG3JJP-_ /edit".to_string()
                }
            ],
            reverse: true,
        }
    ]
}

const VUE: Technology = Technology {
    name: "Vue 3",
    url: "https://vuejs.org/",
};
const NUXT: Technology = Technology {
    name: "Nuxt.js",
    url: "https://nuxt.com/",
};
const CYPRESS: Technology = Technology {
    name: "Cypress",
    url: "https://www.cypress.io/",
};
const SANITY: Technology = Technology {
    name: "Sanity CMS",
    url: "https://www.sanity.io/",
};
const REACT: Technology = Technology {
    name: "React",
    url: "https://react.dev/",
};
const REDUX: Technology = Technology {
    name: "Redux Toolkit",
    url: "https://redux-toolkit.js.org/",
};
const FSD: Technology = Technology {
    name: "FSD",
    url: "https://feature-sliced.design/",
};
const FORMIK: Technology = Technology {
    name: "Formik",
    url: "https://formik.org/",
};
const TS: Technology = Technology {
    name: "TypeScript",
    url: "https://www.typescriptlang.org/",
};
const NODEJS: Technology = Technology {
    name: "Node.js",
    url: "https://nodejs.org/en",
};
const NEST: Technology = Technology {
    name: "NestJS",
    url: "https://nestjs.com/",
};
const MONGODB: Technology = Technology {
    name: "MongoDB",
    url: "https://www.mongodb.com/",
};
const MONGOOSE: Technology = Technology {
    name: "Mongoose",
    url: "https://mongoosejs.com/",
};
const SOCKETIO: Technology = Technology {
    name: "Socket.io",
    url: "https://socket.io/",
};
const NEXTJS: Technology = Technology {
    name: "Next.js",
    url: "https://nextjs.org/",
};
const JEST: Technology = Technology {
    name: "Jest",
    url: "https://jestjs.io/",
};
const RTL: Technology = Technology {
    name: "React Testing Library",
    url: "https://testing-library.com/docs/react-testing-library/intro/",
};
const STORYBOOK: Technology = Technology {
    name: "Storybook",
    url: "https://storybook.js.org/",
};
const YUP: Technology = Technology {
    name: "Yup",
    url: "https://github.com/jquense/yup",
};
const LOKI: Technology = Technology {
    name: "Loki",
    url: "https://storybook.js.org/addons/loki",
};
const EXPRESS: Technology = Technology {
    name: "Express",
    url: "https://expressjs.com/",
};
const MARIADB: Technology = Technology {
    name: "MariaDB",
    url: "https://mariadb.org/",
};
const ERD: Technology = Technology {
    name: "ERD",
    url: "https://erdplus.com/",
};
const BOOTSTRAP: Technology = Technology {
    name: "React-Bootstrap",
    url: "https://react-bootstrap.github.io/",
};
const ANGULAR: Technology = Technology {
    name: "Angular",
    url: "https://angular.io/",
};
const RXJS: Technology = Technology {
    name: "RxJS",
    url: "https://rxjs.dev/",
};
const FULLCALENDAR: Technology = Technology {
    name: "FullCalendar",
    url: "https://fullcalendar.io/",
};
const MOBX: Technology = Technology {
    name: "MobX",
    url: "https://mobx.js.org/",
};
const I18N: Technology = Technology {
    name: "i18n",
    url: "https://www.i18next.com/",
};
const RLEAFLET: Technology = Technology {
    name: "React Leaflet",
    url: "https://react-leaflet.js.org/",
};
const CSS: Technology = Technology {
    name: "CSS",
    url: "https://www.w3.org/Style/CSS/Overview.en.html",
};
const HTML5: Technology = Technology {
    name: "HTML",
    url: "https://html.com/",
};
const UNITY: Technology = Technology {
    name: "Unity",
    url: "https://unity.com/",
};
const CSHARP: Technology = Technology {
    name: "C#",
    url: "https://learn.microsoft.com/en-us/dotnet/csharp/",
};
const ARDUINO: Technology = Technology {
    name: "Arduino",
    url: "https://www.arduino.cc/reference/en/",
};
const AZURE: Technology = Technology {
    name: "Azure",
    url: "https://azure.microsoft.com/",
};
const FRITZING: Technology = Technology {
    name: "Fritzing",
    url: "https://fritzing.org/",
};
const DRAWIO: Technology = Technology {
    name: "Draw.io",
    url: "https://app.diagrams.net/",
};
const DOMOTICZ: Technology = Technology {
    name: "Domoticz",
    url: "https://www.domoticz.com/",
};
const PROWL: Technology = Technology {
    name: "Prowl",
    url: "https://www.prowlapp.com/",
};
