<template>
    <a v-if="isExternal" :href="url.substring(1)" target="_blank">
        <slot />
    </a>
    <router-link v-else :to="to">
        <slot />
    </router-link>
</template>
<script setup lang="ts">
import { computed } from 'vue'
import type { RouterLinkProps } from 'vue-router'

export interface Props extends RouterLinkProps {
    url: string
};
const props = defineProps<Props>();
const isExternal = computed((): boolean => props.url.startsWith("@"));
</script>
