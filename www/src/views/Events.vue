<script>
import { computed, reactive, ref } from "vue";
import queryDB from '@/composables/queryDB';
import vueTable from '@/components/plugins/VueTable.vue';
import { useAppVariableStore } from '@/stores/app-variable';

const appVariable = useAppVariableStore();

export default {
	components: { vueTable },
	data() {
		return {
			rendered: true,
			searchTerm: ref(''),
			keywords: reactive([]),
			events: reactive([]),
			eventsTable: reactive({
				isLoading: true,
				columns: [
					{
						label: "ID",
						field: "id",
						width: "1%",
						sortable: true,
						isKey: true,
						columnStyles: { "text-align": "center" },
						display: function (row) {
							return '<a href="/event/' + row[this.field] + '">#' + row[this.field] + '</a>'; // FIXME: Sanitize this
						}
					},
					{
						label: "Timestamp",
						field: "timestamp",
						width: "10%",
						sortable: true,
						columnStyles: { "text-align": "center" },
						display: function (row) {
							let d = new Date(row[this.field] * 1000);
							return `${d.toLocaleString('es-ES')}`;
						},
					},
					{
						label: "Type",
						field: "type",
						width: "3%",
						sortable: true,
						columnStyles: { "text-align": "center" },
						display: function (row) {
							return '<span class="badge border border-secondary text-secondary px-2 pt-5px pb-5px rounded fs-12px d-inline-flex align-items-center">' + row[this.field].toUpperCase() + '</span>'; // FIXME: Sanitize this
						}
					},
					{
						label: "Template",
						field: "template",
						width: "10%",
						sortable: true,
					},
					{
						label: "Source",
						field: "source",
						width: "13%",
						sortable: true,
						columnClasses: ['truncated'],
						display: function (row) {
							return '<a href="' + row[this.field] + '">' + row[this.field] + '</a>'; // FIXME: Sanitize this
						},
					},
					{
						label: "Data",
						field: "data",
						width: "50%",
						columnClasses: ['truncated'],
					}
				],
				rows: [],
				totalRecordCount: 0,
				sortable: {
					order: "id",
					sort: "desc",
				},
			})
		}
	},
	mounted() {
		let sortmode = (a, b) => b.id - a.id;
		queryDB('query{keywords{id,timestamp:createdAt,type,value,lastConsulted}events{id,timestamp:createdAt,template,type,source,data}}',
			(data) => {
				this.keywords = data.keywords.sort(sortmode);
				this.events = data.events.sort(sortmode);
				this.eventsTable.rows = this.events;
				this.eventsTable.totalRecordCount = this.eventsTable.rows.length;
				this.eventsTable.isLoading = false;
			});
	},
	methods: {
		filterEvents() {
			let events = this.events;
			let search = this.searchTerm.toLowerCase();
			// create the dorks
			let dorks = search.split(' ').filter(d => d.length > 0);
			// filter the events
			dorks.forEach(dork => {
				let dsl = dork.split(':', 2);
				if (dsl.length == 2) {
					let field = dsl[0];
					let value = dsl[1];
					events = events.filter(event => {
						switch (field) {
							case "id":
								return event[field] == value;
							case "timestamp":
								return new Date(event[field] * 1000).toLocaleString('es-ES').includes(value);
							default:
								return event[field].toLowerCase().includes(value);
						}
					});
				} else {
					events = events.filter(e => e.source.toLowerCase().includes(dork) || e.data.toLowerCase().includes(dork));
				}
			});
			// udpate the table rows
			this.eventsTable.rows = events;
			this.eventsTable.totalRecordCount = this.eventsTable.rows.length;
		},
	}
}
</script>
<template>
	<div class="row" v-if="rendered">
		<!-- Header -->
		<div class="d-flex align-items-center mb-md-3 mb-2">
			<!-- Title -->
			<div class="flex-fill">
				<h1 class="page-header mb-0">Dashboard</h1>
			</div>
			<!-- Action -->
			<div class="ms-auto">
				<a href="#" data-bs-toggle="modal" class="btn btn-outline-theme">
					<i class="fa fa-download me-1"></i>
					Export
				</a>
			</div>
		</div>

		<!-- Description -->
		<div class="mb-md-4 mb-3 d-md-flex">
			<div class="ms-md-0 mt-md-0 mt-2">
				<i class="fa fa-key fa-fw fa-lg me-1 text-theme"></i>
				{{ keywords.filter(k => k.type === 'TEXT').length }} keyword(s)
			</div>
			<div class="ms-md-4 mt-md-0 mt-2">
				<i class="fa fa-lock fa-fw fa-lg me-1 text-theme"></i>
				{{ keywords.filter(k => k.type === 'CREDENTIAL').length }} credential(s)
			</div>
			<div class="ms-md-4 mt-md-0 mt-2">
				<i class="fa fa-globe fa-fw fa-lg me-1 text-theme"></i>
				{{ keywords.filter(k => k.type === 'DOMAIN').length }} domain(s)
			</div>
			<div class="ms-md-4 mt-md-0 mt-2">
				<i class="fa fa-envelope fa-fw fa-lg me-1 text-theme"></i>
				{{ keywords.filter(k => k.type === 'EMAIL').length }} email(s)
			</div>
			<div class="ms-md-4 mt-md-0 mt-2">
				<i class="fa fa-server fa-fw fa-lg me-1 text-theme"></i>
				{{ keywords.filter(k => k.type === 'IP').length }} IP(s)
			</div>
			<div class="ms-md-4 mt-md-0 mt-2">
				<i class="fa fa-phone fa-fw fa-lg me-1 text-theme"></i>
				{{ keywords.filter(k => k.type === 'PHONE').length }} phone(s)
			</div>
			<div class="ms-md-4 mt-md-0 mt-2">
				<i class="fa fa-link fa-fw fa-lg me-1 text-theme"></i>
				{{ keywords.filter(k => k.type === 'URL').length }} URL(s)
			</div>
			<div class="ms-md-4 mt-md-0 mt-2">
				<i class="fa fa-user fa-fw fa-lg me-1 text-theme"></i>
				{{ keywords.filter(k => k.type === 'USERNAME').length }} username(s)
			</div>
		</div>

		<!-- Display table -->
		<card>
			<!-- Events -->
			<div class="tab-content p-4">

				<!-- Search bar -->
				<div class="input-group mb-4">
					<!-- Input -->
					<div class="flex-fill position-relative">
						<div class="input-group">
							<input v-model="searchTerm" @input="filterEvents" type="text"
								class="form-control px-35px rounded" placeholder="Search event..." />
							<div class="input-group-text position-absolute top-0 bottom-0 bg-none border-0 start-0"
								style="z-index:1">
								<i class="fa fa-search opacity-5"></i>
							</div>
						</div>
					</div>

					<!-- Action -->
					<div class="ms-4">
						<a href="#" data-bs-toggle="modal" class="btn btn-outline-theme">
							<i class="fa fa-download me-1"></i>
							Debug
						</a>
					</div>
				</div>

				<!-- Event list -->
				<div class="table-responsive">

					<vue-table class="vue-table" :is-static-mode="true" :is-fixed-first-column="true"
						:columns="eventsTable.columns" :rows="eventsTable.rows" :total="eventsTable.totalRecordCount"
						:sortable="eventsTable.sortable" />
				</div>
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

.vtl-tbody-td {
	background-color: transparent !important;
}
</style>