<script setup lang="ts">
import Link from "@/components/Link.vue";
import { useRouter } from "vue-router";

export interface Props {
  icon: string | undefined;
  text: string;
  url: string;
}
defineProps<Props>();

function isActive(url: string) {
  return url == useRouter().currentRoute.value.path;
}
</script>
<template>
  <div class="menu-item" :class="{ active: isActive(url) }">
    <Link class="menu-link" :url="url" :to="url">
      <span class="menu-icon" v-if="icon">
        <i :class="icon"></i>
      </span>
      <span class="menu-text">{{ text }}</span>
    </Link>
  </div>
</template>