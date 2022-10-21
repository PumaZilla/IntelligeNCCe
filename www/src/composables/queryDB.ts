const API_ENDPOINT: string = "/graphql";

export default async function queryDB(query: string, cb: Function): Promise<{}> {
  return fetch(API_ENDPOINT, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ query: query }),
  })
    .then((res) => res.json())
    .then((res) => {
      if (res.errors) {
        console.log(res.errors);
        return {};
      }
      return cb(res.data);
    })
    .catch((err) => console.log(err));
}
