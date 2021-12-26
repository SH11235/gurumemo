import { FC } from 'react';

type Props = {
	onClick: () => void
};

export const SearchButton: FC<Props> = props => {
	const { onClick } = props;
	return (
		<>
		<button onClick={onClick}>Search</button>
		</>
	);
};
