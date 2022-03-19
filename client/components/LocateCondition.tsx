import React, { FC } from 'react';
import styles from '../styles/LocateCondition.module.css';

type Props = {
	word: string,
	onChange: (e: React.ChangeEvent<HTMLInputElement>) => void
};

export const LocateCondition: FC<Props> = (props) => {
    const { word, onChange } = props;
    return (
        <>
            <input placeholder="40.12345" type="text" value={word} onChange={onChange} />
        </>
    )
}
