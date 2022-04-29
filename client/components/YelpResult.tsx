import { FC } from 'react';
import { ApiResult } from "../api_request/yelpBusinessSerachAPI";
import { Card, CardMedia, CardActionArea, CardContent } from '@material-ui/core';
import styles from "../styles/YelpResult.module.css";

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
            <div className={styles.container}>
                {
                    result.businesses.map((item: any, index: number) =>
                        <Card key={index + item.name} className={styles.card}>
                            <CardActionArea href={item.url}>
                                <CardContent className={styles.title}>
                                    <a href={item.url}>
                                        {item.alias ? item.alias: item.name}
                                    </a>
                                </CardContent>
                                <CardMedia
                                    component="img"
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
