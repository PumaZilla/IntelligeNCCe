<template>
    <div class="table-responsive">
        <VueTableLite class="vue-table" :is-static-mode="true" :is-fixed-first-column="true" :columns="columns"
            :rows="rows" :total="rows.length" :sortable="sortSystem" />
    </div>
</template>
<script setup lang="ts">
import { computed } from "vue";
import VueTableLite from "vue3-table-lite/ts";

export interface Props {
    rows: any[];
    columns: any[];
    orderBy: string;
    sortDesc?: boolean;
};
const props = defineProps<Props>();
const sortSystem = computed(() => ({ order: props.orderBy, sort: props.sortDesc ? "desc" : "asc" }));
</script>
<style>

</style>