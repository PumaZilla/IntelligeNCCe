<script>
import { computed, reactive, ref } from "vue";
import queryDB from "@/composables/queryDB";
import { useAppVariableStore } from "@/stores/app-variable";

import Title from "@/components/Title.vue";
import EventsTab from "@/components/EventsTab.vue";
import KeywordsTab from "@/components/KeywordsTab.vue";

const appVariable = useAppVariableStore();

export default {
  components: { Title, EventsTab, KeywordsTab },
  data() {
    return {
      rendered: true,
    };
  },
};
</script>
<template>
  <div class="row" v-if="rendered">
    <Title title="Explorer" />

    <!-- Description
    <div class="mb-md-4 mb-3 d-md-flex">
      <div class="ms-md-0 mt-md-0 mt-2">
        <i class="fa fa-key fa-fw fa-lg me-1 text-theme"></i>
        {{ keywords.filter((k) => k.type === "TEXT").length }} keyword(s)
      </div>
      <div class="ms-md-4 mt-md-0 mt-2">
        <i class="fa fa-lock fa-fw fa-lg me-1 text-theme"></i>
        {{ keywords.filter((k) => k.type === "CREDENTIAL").length }}
        credential(s)
      </div>
      <div class="ms-md-4 mt-md-0 mt-2">
        <i class="fa fa-globe fa-fw fa-lg me-1 text-theme"></i>
        {{ keywords.filter((k) => k.type === "DOMAIN").length }} domain(s)
      </div>
      <div class="ms-md-4 mt-md-0 mt-2">
        <i class="fa fa-envelope fa-fw fa-lg me-1 text-theme"></i>
        {{ keywords.filter((k) => k.type === "EMAIL").length }} email(s)
      </div>
      <div class="ms-md-4 mt-md-0 mt-2">
        <i class="fa fa-server fa-fw fa-lg me-1 text-theme"></i>
        {{ keywords.filter((k) => k.type === "IP").length }} IP(s)
      </div>
      <div class="ms-md-4 mt-md-0 mt-2">
        <i class="fa fa-phone fa-fw fa-lg me-1 text-theme"></i>
        {{ keywords.filter((k) => k.type === "PHONE").length }} phone(s)
      </div>
      <div class="ms-md-4 mt-md-0 mt-2">
        <i class="fa fa-link fa-fw fa-lg me-1 text-theme"></i>
        {{ keywords.filter((k) => k.type === "URL").length }} URL(s)
      </div>
      <div class="ms-md-4 mt-md-0 mt-2">
        <i class="fa fa-user fa-fw fa-lg me-1 text-theme"></i>
        {{ keywords.filter((k) => k.type === "USERNAME").length }} username(s)
      </div>
    </div>
  -->

    <!-- Display table -->
    <card>
      <!-- Event type selector -->
      <ul class="nav nav-tabs nav-tabs-v2 px-4">
        <li class="nav-item me-3">
          <a href="#events" class="nav-link active px-2" data-bs-toggle="tab"
            >Events</a
          >
        </li>
        <li class="nav-item me-3">
          <a href="#keywords" class="nav-link px-2" data-bs-toggle="tab"
            >Keywords</a
          >
        </li>
      </ul>
      <div class="tab-content p-4">
        <EventsTab class="active" id="events" />
        <KeywordsTab id="keywords" />
      </div>
    </card>
  </div>
</template>
<style>
.truncated {
  max-width: 0;
}

.truncated * {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  max-width: fit-content;
}

.vtl-thead-th {
  text-align: center;
}

.vtl-thead-th,
.vtl-tbody-td {
  background-color: transparent !important;
}
</style>
