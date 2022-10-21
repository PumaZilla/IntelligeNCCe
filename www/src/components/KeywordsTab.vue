<template>
    <Tab :id="id">
        <SearchBar :event="filter" placeholder="Search keywords..." :modal="modal" />
        <KeywordsTable :rows="filteredKeywords" />
    </Tab>
</template>
<script setup lang="ts">
import { onMounted, ref } from "vue";

import KeywordsTable from "@/components/KeywordsTable.vue";
import SearchBar from "@/components/SearchBar.vue";
import Tab from "@/components/Tab.vue";
import queryDB from "@/composables/queryDB";

export interface Props {
    id: string,
}
defineProps<Props>();

const modal = {
    link: "#",
    text: "Add",
    icon: "fas fa-plus",
};

let keywords = ref([]);
let filteredKeywords = ref([]);
onMounted(() => queryDB("query{keywords{id,timestamp:createdAt,type,value,lastConsulted}}", (data: any) => {
    keywords.value = data.keywords
    filteredKeywords.value = data.keywords
}));

const filter = (ev: Event) => {
    let filtered = keywords.value;
    const query = (ev.target as HTMLInputElement).value.toLowerCase();
    // create the dorks
    const dorks = query.split(" ").filter((d) => d.length > 0);
    // filter the events
    dorks.forEach((dork) => {
        const dsl = dork.split(":", 2);
        if (dsl.length == 2) {
            const field = dsl[0];
            const value = dsl[1];
            filtered = filtered.filter((keyword: any) => {
                switch (field) {
                    case "id":
                        return keyword[field] == value;
                    case "timestamp":
                    case "lastConsulted":
                        return new Date(keyword[field] * 1000)
                            .toLocaleString("es-ES")
                            .includes(value);
                    default:
                        return keyword[field].toLowerCase().includes(value);
                }
            });
        } else {
            filtered = filtered.filter((keyword: any) => keyword.value.toLowerCase().includes(dork));
        }
    });
};
</script>