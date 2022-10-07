<script>
import queryDB from '@/composables/queryDB';
import vueTable from '@/components/plugins/VueTable.vue';
import { useAppVariableStore } from '@/stores/app-variable';

const appVariable = useAppVariableStore();

export default {
	components: { vueTable },
	data() {
		return {
			rendered: true,
			events: [],
			eventsTable: {
				isLoading: false,
				columns: [
					{
						label: "ID",
						field: "id",
						width: "3%",
						sortable: true,
						isKey: true,
					},
					{
						label: "Timestamp",
						field: "timestamp",
						width: "10%",
						sortable: true,
					},
					{
						label: "Template",
						field: "template",
						width: "10%",
						sortable: true,
					},
					{
						label: "Type",
						field: "type",
						width: "10%",
						sortable: true,
					},
					{
						label: "Source",
						field: "source",
						width: "25%",
						sortable: true,
					},
					{
						label: "Data",
						field: "data",
						width: "50%",
					}
				],
				rows: [],
				totalRecordCount: 0,
				sortable: {
					order: "id",
					sort: "desc",
				},
			}
		}
	},
	mounted() {
		let sortmode = (a, b) => b.id - a.id;
		queryDB('query{event{id,timestamp:createdAt,template,type,source,data}}',
			(data) => {
				this.events = data.event.sort(sortmode);
				this.eventsTable.rows = this.events;
				this.eventsTable.totalRecordCount = this.events.length;
			});
	},
	methods: {
		truncate(element) {
			if (typeof element !== 'string') return element;

			let n = 120;
			return element.substr(0, n - 1) + (element.length > n ? '...' : '')
		},
		timestamp(element) {
			let d = new Date(element * 1000);
			return `${d.toLocaleString('es-ES')}`;
		},
		getTypes(elements) {
			return [...new Set(elements.map(k => k.type))];
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
				<h1 class="page-header mb-0">Events</h1>
			</div>
			<!-- Action -->
			<div class="ms-auto">
				<a href="#" data-bs-toggle="modal" class="btn btn-outline-theme">
					<i class="fa fa-download me-1"></i>
					Debug
				</a>
			</div>
		</div>

		<!-- Display table -->
		<card>
			<div class="tab-content p-4">
				<!-- Events -->
				<!-- Search bar -->
				<div class="input-group mb-4">
					<!-- Input -->
					<div class="flex-fill position-relative">
						<div class="input-group">
							<input type="text" class="form-control px-35px" placeholder="Search event..." />
							<div class="input-group-text position-absolute top-0 bottom-0 bg-none border-0 start-0"
								style="z-index:1">
								<i class="fa fa-search opacity-5"></i>
							</div>
						</div>
					</div>
				</div>

				<!-- Event list -->
				<div class="table-responsive">

					<vue-table class="vue-table" :is-static-mode="true" :columns="eventsTable.columns"
						:rows="eventsTable.rows" :total="eventsTable.totalRecordCount"
						:sortable="eventsTable.sortable" />

				</div>
			</div>
		</card>
	</div>
</template>