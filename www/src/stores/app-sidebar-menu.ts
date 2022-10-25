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
        url: "#",
        icon: "fas fa-microchip",
        text: "Dashboard",
      },
      {
        url: "#",
        icon: "fas fa-handshake",
        text: "Projects",
      },
      {
        url: "/",
        icon: "fas fa-compass",
        text: "Explorer",
      },
      {
        url: "#",
        icon: "fas fa-paste",
        text: "Designer",
      },
      
      {
        is_header: true,
        text: "Administration",
      },
      {
        url: "#",
        icon: "fas fa-users",
        text: "User Management",
      },
      {
        url: "#",
        icon: "fas fa-server",
        text: "Server Status",
      },
      {
        is_header: true,
        text: "Account",
      },
      {
        url: "#",
        icon: "fas fa-bell",
        text: "Notifications",
      },
      {
        url: "#",
        icon: "fas fa-cog",
        text: "Settings",
      },
      {
        url: "#",
        icon: "fas fa-bed",
        text: "Logout",
      },
      
      {
        is_header: true,
        text: "Resources",
      },
      {
        url: "#",
        icon: "fas fa-book",
        text: "Documentation",
      },
      {
        url: "@https://git.pentest.ngs/kike.fontan/intelligencce",
        icon: "fa fa-code-branch",
        text: "Repository",
      },
    ];
  },
});
