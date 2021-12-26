import { yelpBusinessSearchAPIURL } from "../setting";

export type conditions = {
  latitude?: string;
  longitude?: string;
  range?: string;
};

export const yelpBusinessSearchAPI = async (
  cond: conditions,
) => {
  try {
    const params: string[] = [
      `latitude=${cond.latitude}`,
      `longitude=${cond.longitude}`,
      `radius=${cond.range}`,
    ];
    const res = await fetch(`${yelpBusinessSearchAPIURL}?${params.join("&")}`, {
      mode: "cors",
    });
    let results = await res.json();
    console.log(results); // TODO resultsを状態管理して画面描画する
  } catch (error) {
    console.error(error);
  }
};
