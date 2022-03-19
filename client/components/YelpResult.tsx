import { FC } from 'react';

type Props = {
	result: any
};

export const YelpResult: FC<Props> = props => {
    const { result } = props;


	
    if (result.businesses.length > 0) {
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
