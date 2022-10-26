<template>
    <Modal id="add_keyword" title="New keyword" :callback="save">
        <div class="alert alert-muted">
            <b>Please note:</b> 
            Some templates may only run according to the type of keyword specified.  
            <a href="#" class="alert-link">Learn more.</a>
        </div>

        <div class="mb-3">
            <div class="row" id="new_keyword_form">
                <div class="col-8">
                    <label class="form-label">Value</label>
                    <input class="form-control" placeholder="Example: Intelligencce" />
                </div>
                <div class="col-4">
                    <label class="form-label">Type</label>
                    <VueSelect :options="['TEXT','DOMAIN','IP']" placeholder="Select a type"/>
                </div>
            </div>
        </div>
    </Modal>
</template>
<script setup lang="ts">
import Modal from "@/components/Modal.vue";
import VueSelect from '@/components/plugins/VueSelect.vue';
import queryDB from "@/composables/queryDB";

export interface Props {}
defineProps<Props>();

const save = (payload: Event) => {
    const form = document.getElementById('new_keyword_form');
    let value = form?.querySelector('input')?.value.trim();
    if (!value || value?.length == 0) {
        alert('Please enter a value for the keyword'); // FIXME: Error message in the modal
        return;
    }
    let type = form?.querySelector('select')?.value;
    type = type ? type : 'TEXT';
    queryDB(`mutation{keyword:createKeyword(keyword:{value:"${value}",type:${type}}){id}}`, (data: any) => { // FIXME: INJECTION!
        if (data.keyword.id) {
            window.location.reload();
        } else {
            alert('An error occurred while creating the keyword'); // FIXME: Error message in the modal
        }
    });
};
</script>