<script>
import queryDB from '@/composables/queryDB';
import { useAppVariableStore } from '@/stores/app-variable';

const appVariable = useAppVariableStore();

export default {
	data() {
		return {
			events: [],
			eventKeys: ['id', 'timestamp', 'template', 'type', 'source', 'data'],
			keywords: [],
			keywordKeys: ['id', 'timestamp', 'lastConsulted', 'type', 'value'],
			rendered: true,
		}
	},
	mounted() {
		let sortmode = (a, b) => b.id - a.id;
		queryDB('query{keywords{id,timestamp:createdAt,type,value,lastConsulted}events{id,timestamp:createdAt,template,type,source,data}}',
			(data) => {
				this.events = data.events.sort(sortmode);
				this.keywords = data.keywords.sort(sortmode);
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
		<!-- Subheader -->
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
			<!-- Event type selector -->
			<ul class="nav nav-tabs nav-tabs-v2 px-4">
				<li class="nav-item me-3"><a href="#events" class="nav-link active px-2" data-bs-toggle="tab">Events</a>
				</li>
				<li class="nav-item me-3"><a href="#keywords" class="nav-link px-2" data-bs-toggle="tab">Keywords</a>
				</li>
			</ul>
			<div class="tab-content p-4">
				<!-- Events -->
				<div class="tab-pane fade show active" id="events">
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
						<!-- Action -->
						<button class="btn btn-outline-default dropdown-toggle rounded-0" type="button"
							data-bs-toggle="dropdown">
							<span class="d-none d-md-inline">Filter by type</span>
							<span class="d-inline d-md-none">
								<i class="fa fa-filter"></i>
							</span>
							&nbsp;
						</button>
						<div class="dropdown-menu">
							<a class="dropdown-item" href="#" v-for="t in getTypes(events)">{{ t }}</a>
						</div>
					</div>
					<!-- Event list -->
					<div class="table-responsive">
						<table class="table table-hover text-nowrap">
							<thead class="table-dark">
								<tr>
									<th class="border-top-0 pt-2 pb-2 align-middle"></th>
									<th class="border-top-0 pt-2 pb-2 align-middle text-center" v-for="k in eventKeys">
										{{ k.charAt(0).toUpperCase() + k.slice(1) }}
									</th>
								</tr>
							</thead>
							<tbody>
								<tr v-for="event in events">
									<!-- Checkbox -->
									<td class="align-middle action">
										<div class="form-check">
											<input type="checkbox" class="form-check-input" :id="'event#' + event.id">
											<label class="form-check-label" :for="'event#' + event.id"></label>
										</div>
									</td>
									<td class="align-middle text-center" v-for="k in eventKeys">
										<RouterLink to="#unimplemented" v-if="k === 'id'">
											#{{ event[k] }}
										</RouterLink>
										<span v-else-if="k === 'timestamp'">
											{{ timestamp(event[k]) }}
										</span>
										<span v-else-if="k === 'template'"
											class="badge border border-secondary text-secondary px-2 pt-5px pb-5px rounded fs-12px d-inline-flex align-items-center">
											{{ truncate(event[k]) }}
										</span>
										<a v-else-if="k === 'source'" :href="event[k]" class="float-start">
											{{ truncate(event[k]) }}
										</a>
										<span v-else class="float-start">
											{{ truncate(event[k]) }}
										</span>
									</td>
								</tr>
							</tbody>
						</table>
					</div>
				</div>
				<!-- Keywords -->
				<div class="tab-pane fade" id="keywords">
					<!-- Search bar -->
					<div class="input-group mb-4">
						<!-- Input -->
						<div class="flex-fill position-relative">
							<div class="input-group">
								<input type="text" class="form-control px-35px" placeholder="Search keyword..." />
								<div class="input-group-text position-absolute top-0 bottom-0 bg-none border-0 start-0"
									style="z-index:1">
									<i class=" fa fa-search opacity-5"></i>
								</div>
							</div>
						</div>
						<!-- Action -->
						<button class="btn btn-outline-default dropdown-toggle rounded-0" type="button"
							data-bs-toggle="dropdown">
							<span class="d-none d-md-inline">Filter by type</span>
							<span class="d-inline d-md-none">
								<i class="fa fa-filter"></i>
							</span>
							&nbsp;
						</button>
						<div class="dropdown-menu">
							<a class="dropdown-item" href="#" v-for="t in getTypes(keywords)">{{ t }}</a>
						</div>
					</div>
					<!-- Keyword list -->
					<div class="table-responsive">
						<table class="table table-hover text-nowrap">
							<thead class="table-dark">
								<tr>
									<th class="border-top-0 pt-2 pb-2 align-middle"></th>
									<th class="border-top-0 pt-2 pb-2 align-middle text-center"
										v-for="k in keywordKeys">
										{{ k.charAt(0).toUpperCase() + k.slice(1) }}
									</th>
								</tr>
							</thead>
							<tbody>
								<tr v-for="keyword in keywords">
									<!-- Checkbox -->
									<td class="align-middle action">
										<div class="form-check">
											<input type="checkbox" class="form-check-input"
												:id="'keyword#' + keyword.id">
											<label class="form-check-label" :for="'keyword#' + keyword.id"></label>
										</div>
									</td>
									<td class="align-middle text-center" v-for="k in keywordKeys">
										<RouterLink to="#unimplemented" v-if="k === 'id'">
											#{{ keyword[k] }}
										</RouterLink>
										<span v-else-if="k === 'timestamp' || k === 'lastConsulted'">
											{{ timestamp(keyword[k]) }}
										</span>
										<span v-else-if="k === 'type'"
											class="badge border border-secondary text-secondary px-2 pt-5px pb-5px rounded fs-12px d-inline-flex align-items-center">
											{{ truncate(keyword[k]) }}
										</span>
										<a v-else-if="k === 'source'" :href="keyword[k]" class="float-start">
											{{ truncate(keyword[k]) }}
										</a>
										<span v-else class="float-start">
											{{ truncate(keyword[k]) }}
										</span>
									</td>
								</tr>
							</tbody>
						</table>
					</div>
				</div>
			</div>
		</card>
	</div>
</template>