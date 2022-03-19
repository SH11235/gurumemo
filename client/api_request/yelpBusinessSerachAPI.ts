import { yelpBusinessSearchAPIURL } from "../setting";

export type ApiResult = {
    total: number;
    businesses: {}[];
    region: {};
    isError: boolean;
};

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
        let results: ApiResult = await res.json();
        results.isError = false;
        return results;
    } catch (error) {
        const results: ApiResult = {
            total: 0,
            businesses: [],
            region: {},
            isError: true
        };
        console.error("Backendへのリクエストでエラーになりました");
        console.error(error);
        return results;
    }
};
