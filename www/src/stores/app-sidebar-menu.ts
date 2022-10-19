import { defineStore } from "pinia";

export const useAppSidebarMenuStore = defineStore({
  id: "appSidebarMenu",
  state: () => {
    return [
      {
        text: "Navigation",
        is_header: true,
      },
      {
        url: "/",
        icon: "bi bi-cpu",
        text: "Dashboard",
      },
      {
        text: "Resources",
        is_header: true,
      },
      {
        url: "@/i_understand_that_this_is_against_security",
        icon: "fa fa-code-branch",
        text: "GraphQL",
      },
      {
        url: "@https://git.pentest.ngs/kike.fontan/intelligencce",
        icon: "fa fa-code-branch",
        text: "Repository",
      },
    ];
  },
});
