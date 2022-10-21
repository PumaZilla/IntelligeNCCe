import { defineStore } from "pinia";

export const useAppSidebarMenuStore = defineStore({
  id: "appSidebarMenu",
  state: () => {
    return [
      {
        is_header: true,
        text: "Navigation",
      },
      {
        url: "/",
        icon: "bi bi-cpu",
        text: "Dashboard",
      },
      {
        url: "/explore",
        icon: "far fa-compass",
        text: "Explore",
      },
      {
        is_header: true,
        text: "Resources",
      },
      {
        url: "@https://git.pentest.ngs/kike.fontan/intelligencce",
        icon: "fa fa-code-branch",
        text: "Repository",
      },
    ];
  },
});
