<template>
    <div class="tab-pane fade show" :id="id">
        <slot />
    </div>
</template>
<script setup lang="ts">
export interface Props {
    id: string;
}
defineProps<Props>();
</script>