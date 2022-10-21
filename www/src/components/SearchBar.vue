<template>
    <div class="input-group mb-4">
        <div class="flex-fill position-relative">
            <div class="input-group">
                <input @input="event" type="text" class="form-control px-35px rounded"
                    :placeholder="placeholder" />
                <div class="input-group-text position-absolute top-0 bottom-0 bg-none border-0 start-0"
                    style="z-index: 1">
                    <i class="fa fa-search opacity-5"></i>
                </div>
            </div>
        </div>

        <ModalButton v-if="modal !== undefined" v-bind="modal" class="ms-4" />
    </div>
</template>
<script setup lang="ts">
import ModalButton from "@/components/ModalButton.vue";

export interface Props {
    event: (payload: Event) => void;
    placeholder?: string;
    modal?: {
        link: string;
        text: string;
        icon?: string;
    };
}
defineProps<Props>();
</script>