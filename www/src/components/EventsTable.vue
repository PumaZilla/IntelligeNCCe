<template>
    <Table orderBy="id" :sortDesc="true" :columns="columns" :rows="rows" />
</template>
<script setup lang="ts">
import Table from "@/components/Table.vue";

export interface Props {
    rows: any[];
};
defineProps<Props>();

const defaultLocale = "en-GB"; // FIXME: Get this from the user's browser or from store
// FIXME: New file with this functions
const linkify = (link: string, display?: string) => `<a href="${link}">${display || link}</a>`;
const badgify = (text: string) => `<span class="badge border border-secondary text-secondary px-2 pt-5px pb-5px rounded fs-12px d-inline-flex align-items-center">${text.toUpperCase()}</span>`;
const timestampify = (unixtime: number) => `${new Date(unixtime * 1000).toLocaleString(defaultLocale)}`;

const columns = [
    {
        isKey: true, label: "ID", field: "id", width: "1%", sortable: true,
        display: (row: any) => linkify(`/raw/${row.id}`, `#${row.id}`),
    },
    {
        label: "Timestamp", field: "timestamp", width: "11%", sortable: true,
        display: (row: any) => timestampify(row.timestamp),
    },
    {
        label: "Type", field: "type", width: "3%", sortable: true,
        display: (row: any) => badgify(row.type),
    },
    {
        label: "Template", field: "template", width: "8%", sortable: true,
    },
    {
        label: "Source", field: "source", width: "40%", sortable: true, columnClasses: ["truncated"],
        display: (row: any) => linkify(row.source),
    },
    {
        label: "Keywords", field: "source", width: "20%", sortable: true, columnClasses: ["truncated"],
        display: (row: any) => row.keywords.map((e: any) => e.value).join(", "),
    },
]
</script>