<template>
    <div>
        <a :href="link" data-bs-toggle="modal" class="btn btn-outline-theme">
            <i v-if="icon !== undefined" class="me-1" :class="icon"></i>
            {{ text }}
        </a>
    </div>
</template>
<script setup lang="ts">
export interface Props {
    link: string;
    text: string;
    icon?: string;
}
defineProps<Props>();
</script>