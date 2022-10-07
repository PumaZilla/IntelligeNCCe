import { defineStore } from "pinia";

export const useAppSidebarMenuStore = defineStore({
  id: "appSidebarMenu",
  state: () => {
    return [{
      'text': 'Navigation',
      'is_header': true
    },{
      'url': '/',
      'icon': 'bi bi-cpu',
      'text': 'Dashboard'
    },{
      'url': '/events',
      'icon': 'bi bi-alert',
      'text': 'Events'
    }]
  }
});