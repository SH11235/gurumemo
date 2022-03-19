import { yelpBusinessSearchAPIURL } from "../setting";

export type conditions = {
  latitude?: string;
  longitude?: string;
  range?: string;
  limit?: string;
};

export const yelpBusinessSearchAPI = async (
    cond: conditions,
) => {
    try {
        const params: string[] = [
            `latitude=${cond.latitude}`,
            `longitude=${cond.longitude}`,
            `radius=${cond.range}`,
            `limit=${cond.limit}`,
        ];
        const res = await fetch(`${yelpBusinessSearchAPIURL}?${params.join("&")}`, {
            mode: "cors",
        });
        let results = await res.json();
        return results;
    } catch (error) {
        console.error(error);
    }
};
