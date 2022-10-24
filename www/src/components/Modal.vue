<template>
<!-- BEGIN #modalEdit -->
<div class="modal fade" :id="id">
    <div class="modal-dialog">
        <div class="modal-content">
            <div class="modal-header">
                <h5 class="modal-title">{{ title }}</h5>
                <button type="button" class="btn-close" data-bs-dismiss="modal"></button>
            </div>
            <div class="modal-body">
                <slot />
            </div>
            <div class="modal-footer">
                <button type="button" class="btn btn-outline-default" data-bs-dismiss="modal">Close</button>
                <button type="button" class="btn btn-outline-theme" @click="callback">Save changes</button>
            </div>
        </div>
    </div>
</div>
<!-- END #modalEdit -->
</template>
<script setup lang="ts">
export interface Props {
    id: string;
    title: string;
    callback: (payload: Event) => void;
}
defineProps<Props>();
</script>