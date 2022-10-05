<script>
import { useAppVariableStore } from '@/stores/app-variable';
import apexchart from '@/components/plugins/Apexcharts.vue';

const appVariable = useAppVariableStore();

export default {
	components: {
		apexchart
	},
	data() {
		return {
			rendered: true,
			stats: this.getStats()
		}
	},
	methods: {
		getStats() {
			return [
				{
					title: 'INVESTIGATIONS', total: '4.2m',
					info: [{ icon: 'fa fa-chevron-up fa-fw me-1', text: '33.3% more than last week' }, { icon: 'far fa-user fa-fw me-1', text: '45.5% new visitors' }, { icon: 'far fa-times-circle fa-fw me-1', text: '3.25% bounce rate' }],
					chart: {
						height: 30,
						options: { chart: { type: 'bar', sparkline: { enabled: true } }, colors: [appVariable.color.theme], plotOptions: { bar: { horizontal: false, columnWidth: '65%', endingShape: 'rounded' } } },
						series: [{ name: 'Visitors', data: [this.randomNo(), this.randomNo(), this.randomNo(), this.randomNo(), this.randomNo(), this.randomNo(), this.randomNo(), this.randomNo(), this.randomNo(), this.randomNo(), this.randomNo(), this.randomNo(), this.randomNo()] }]
					}
				},
				{
					title: 'EVENTS', total: '$35.2K',
					info: [{ icon: 'fa fa-chevron-up fa-fw me-1', text: '20.4% more than last week' }, { icon: 'fa fa-shopping-bag fa-fw me-1', text: '33.5% new orders' }, { icon: 'fa fa-dollar-sign fa-fw me-1', text: '6.21% conversion rate' }],
					chart: {
						height: 30,
						options: { chart: { type: 'line', sparkline: { enabled: true } }, colors: [appVariable.color.theme], stroke: { curve: 'straight', width: 2 } },
						series: [{ name: 'Visitors', data: [this.randomNo(), this.randomNo(), this.randomNo(), this.randomNo(), this.randomNo(), this.randomNo(), this.randomNo(), this.randomNo(), this.randomNo(), this.randomNo(), this.randomNo(), this.randomNo(), this.randomNo()] }]
					}
				}
			]
		},

		randomNo() {
			return Math.floor(Math.random() * 60) + 30
		},
	}
}
</script>
<template>
	<div class="row" v-if="rendered">
		<!-- BEGIN stats -->
		<div class="col-xl-3 col-lg-6" v-for="stat in stats">
			<!-- BEGIN card -->
			<card class="mb-3">
				<card-body>
					<div class="d-flex fw-bold small mb-3">
						<span class="flex-grow-1">{{ stat.title }}</span>
						<card-expand-toggler />
					</div>
					<div class="row align-items-center mb-2">
						<div class="col-7">
							<h3 class="mb-0">{{ stat.total }}</h3>
						</div>
						<div class="col-5">
							<div class="mt-n3 mb-n2">
								<apexchart :height="stat.chart.height" :options="stat.chart.options"
									:series="stat.chart.series"></apexchart>
							</div>
						</div>
					</div>
					<div class="small text-white text-opacity-50 text-truncate">
						<template v-for="statInfo in stat.info">
							<div>
								<i v-bind:class="statInfo.icon"></i> {{ statInfo.text }}
							</div>
						</template>
					</div>
				</card-body>
			</card>
			<!-- END card -->
		</div>
		<!-- END stats -->
	</div>
</template>