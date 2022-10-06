const API_ENDPOINT = '/graphql';

export default function queryDB(query,cb) {
	return fetch(API_ENDPOINT, {
		method: 'POST',
		headers: {'Content-Type':'application/json'},
		body: JSON.stringify({query: query})
	}).then(res => res.json()).then(res => {
		if (res.errors) {
			console.log(res.errors);
			return {};
		}
		cb(res.data);
	}).catch(err => console.log(err));
}
