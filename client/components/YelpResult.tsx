import { FC } from 'react';
import { ApiResult } from "../api_request/yelpBusinessSerachAPI"

type Props = {
    result: ApiResult
};

export const YelpResult: FC<Props> = props => {
    const { result } = props;

    if (result.isError) {
        return (
            <>
                Backendへのリクエストでエラーになりました.
            </>
        )
    } else if (result.businesses.length > 0) {
        return (
            <div>
                {
                    result.businesses.map((item: any, index: number) =>
                        <p key={index + item.name}>{item.name}</p>)
                }
            </div>
        );
    } else {
        return (
            <>
                No Results.
            </>
        )
    }
};
