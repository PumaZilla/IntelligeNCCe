<script setup lang="ts">
import { useAppSidebarMenuStore } from "@/stores/app-sidebar-menu";
import { useAppOptionStore } from "@/stores/app-option";
import { onMounted, computed } from "vue";

import LinkSidebar from "@/components/LinkSidebar.vue";

const appSidebarMenu = useAppSidebarMenuStore();
const appOption = useAppOptionStore();

function appSidebarMobileToggled() {
  appOption.appSidebarMobileToggled = !appOption.appSidebarMobileToggled;
}

let version: string = '0.0.0';

onMounted(() => {
  //console.log(queryDB('{appVersion}',(data: any) => data.appVersion));

  const handleSidebarMenuToggle = function (menus) {
    menus.map(function (menu) {
      menu.onclick = function (e) {
        e.preventDefault();
        const target = this.nextElementSibling;

        menus.map(function (m) {
          const otherTarget = m.nextElementSibling;
          if (otherTarget !== target) {
            otherTarget.style.display = "none";
            otherTarget.closest(".menu-item").classList.remove("expand");
          }
        });

        const targetItemElm = target.closest(".menu-item");

        if (
          targetItemElm.classList.contains("expand") ||
          (targetItemElm.classList.contains("active") && !target.style.display)
        ) {
          targetItemElm.classList.remove("expand");
          target.style.display = "none";
        } else {
          targetItemElm.classList.add("expand");
          target.style.display = "block";
        }
      };
    });
  };

  const menuBaseSelector = ".app-sidebar .menu > .menu-item.has-sub";
  const submenuBaseSelector = " > .menu-submenu > .menu-item.has-sub";

  // menu
  const menuLinkSelector = menuBaseSelector + " > .menu-link";
  const menus = [].slice.call(document.querySelectorAll(menuLinkSelector));
  handleSidebarMenuToggle(menus);

  // submenu lvl 1
  const submenuLvl1Selector = menuBaseSelector + submenuBaseSelector;
  const submenusLvl1 = [].slice.call(
    document.querySelectorAll(submenuLvl1Selector + " > .menu-link")
  );
  handleSidebarMenuToggle(submenusLvl1);

  // submenu lvl 2
  const submenuLvl2Selector =
    menuBaseSelector + submenuBaseSelector + submenuBaseSelector;
  const submenusLvl2 = [].slice.call(
    document.querySelectorAll(submenuLvl2Selector + " > .menu-link")
  );
  handleSidebarMenuToggle(submenusLvl2);
});
</script>
<template>
  <div id="sidebar" class="app-sidebar">
    <perfect-scrollbar class="app-sidebar-content">

      <!-- Navigation -->
      <div class="menu">
        <template v-for="menu in appSidebarMenu">
          <div class="menu-header" v-if="menu.is_header">{{ menu.text }}</div>
          <LinkSidebar v-else :text="menu.text || ''" :icon="menu.icon" :url="menu.url || '/'"></LinkSidebar> <!-- FIXME: LinkSidebar -->
        </template>
      </div>

      <!-- Footer button -->
      <div class="p-3 px-4 mt-auto">
        <a class="btn d-block btn-outline-theme">
          <i class="fa fa-code-branch me-2 ms-n2 opacity-5"></i>
          Version: {{ version }}
        </a>
      </div>
    </perfect-scrollbar>
  </div>
  <button
    class="app-sidebar-mobile-backdrop"
    v-on:click="appSidebarMobileToggled"
  ></button>
</template>
