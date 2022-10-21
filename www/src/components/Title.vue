<template>
    <div class="d-flex align-items-center mb-md-3 mb-2">
        <div class="flex-fill">
            <h1 class="page-header mb-0">{{ title }}</h1>
        </div>
        <ModalButton v-if="modal !== undefined" v-bind="modal" class="ms-auto" />
    </div>
</template>
<script setup lang="ts">
import ModalButton from "@/components/ModalButton.vue";

export interface Props {
    title: string;
    modal?: {
        link: string;
        text: string;
        icon?: string;
    };
}
defineProps<Props>();
</script>