<template>
    <Tab :id="id">
        <SearchBar :event="filter" placeholder="Search events..." />
        <EventsTable :rows="filteredEvents" />
    </Tab>
</template>
<script setup lang="ts">
import { onMounted, ref } from "vue";

import EventsTable from "@/components/EventsTable.vue";
import SearchBar from "@/components/SearchBar.vue";
import Tab from "@/components/Tab.vue";
import queryDB from "@/composables/queryDB";

export interface Props {
    id: string,
}
defineProps<Props>();

let events = ref([]);
let filteredEvents = ref([]);
onMounted(() => queryDB("query{events{id,timestamp:createdAt,template,type,source,data}}", (data: any) => {
    events.value = data.events
    filteredEvents.value = data.events
}));

const filter = (ev: Event) => {
    let filtered = events.value;
    const query = (ev.target as HTMLInputElement).value.toLowerCase();
    // create the dorks
    const dorks = query.split(" ").filter((d) => d.length > 0);
    // filter the events
    dorks.forEach((dork) => {
        const dsl = dork.split(":", 2);
        if (dsl.length == 2) {
            const field = dsl[0];
            const value = dsl[1];
            filtered = filtered.filter((event: any) => {
                switch (field) {
                    case "id":
                        return event[field] == value;
                    case "timestamp":
                        return new Date(event[field] * 1000)
                            .toLocaleString("es-ES")
                            .includes(value);
                    default:
                        return event[field].toLowerCase().includes(value);
                }
            });
        } else {
            filtered = filtered.filter((event: any) => event.source.toLowerCase().includes(dork) || event.data.toLowerCase().includes(dork));
        }
    });
};
</script>