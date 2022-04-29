import { FC } from 'react';
import { ApiResult } from "../api_request/yelpBusinessSerachAPI";
import { Card, CardMedia, CardActionArea, CardContent } from '@material-ui/core';

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
            <div style={{display: "flex", flexWrap: "wrap", justifyContent: "center"}}>
                {
                    result.businesses.map((item: any, index: number) =>
                        <Card key={index + item.name} style={{margin: "10px"}}>
                            <CardActionArea href={item.url} style={{width:"300px"}}>
                                <CardContent>
                                    <a href={item.url}>
                                        {item.alias ? item.alias: item.name}
                                    </a>
                                </CardContent>
                                <CardMedia
                                    component="img"
                                    height="194"
                                    image={item.image_url}
                                />
                            </CardActionArea>
                        </Card>
                    )
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
