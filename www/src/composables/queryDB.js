const API_ENDPOINT = '/graphql';
const API_DEV_ENDPOINT = 'http://127.0.0.1:1433/graphql';

export default function queryDB() {
	const internalInstance = getCurrentInstance(); 
	const emitter = internalInstance.appContext.config.globalProperties.emitter;

	return emitter;
}
