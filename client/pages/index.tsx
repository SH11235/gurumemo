import type { NextPage } from "next";
import React, { useState } from "react";
import Head from "next/head";
import Image from "next/image";
import styles from "../styles/Home.module.css";
import { LocateCondition } from "../components/LocateCondition";
import { SearchButton } from "../components/SearchButton";
import { YelpResult } from "../components/YelpResult";
import { Conditions, yelpBusinessSearchAPI } from "../api_request/yelpBusinessSerachAPI";
import { ApiResult } from "../api_request/yelpBusinessSerachAPI"

const Home: NextPage = () => {
    let conditions: Conditions = {};
    const [latitudeState, setLatitudeState] = useState(
        conditions.latitude ? conditions.latitude : "35.69059985184279"
    );
    const [longitudeState, setLongitudeState] = useState(
        conditions.longitude ? conditions.longitude : "139.70279058434141"
    );
    const [rangeState, setRangeState] = useState(
        conditions.range ? conditions.range : "100"
    );
    const [limitState, setLimitState] = useState(
        conditions.limit ? conditions.limit : "50"
    );
    const [resultState, setResultState] = useState<ApiResult>(
        {
            total: 0,
            businesses: [],
            region: {},
            isError: false
        }
    );
    const handleLatitudeChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const value = e.target.value;
        setLatitudeState(() => {
            return value;
        });
    };
    const handleLongitudeChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const value = e.target.value;
        setLongitudeState(() => {
            return value;
        });
    };
    const handleRangeChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const value = e.target.value;
        setRangeState(() => {
            return value;
        });
    };
    const handleLimitChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const value = e.target.value;
        setLimitState(() => {
            return value;
        });
    };
    const searchButtonClick = async () => {
        const searchCond: Conditions = {
            latitude: latitudeState,
            longitude: longitudeState,
            range: rangeState,
            limit: limitState
        };
        const result = await yelpBusinessSearchAPI(searchCond);
        setResultState(() => {
            return result;
        });
    };
    return (
        <div className={styles.container}>
            <Head>
                <title>Gurumemo</title>
                <meta name="description" content="Generated by create next app" />
                <link rel="icon" href="/favicon.ico" />
            </Head>
            <main className={styles.main}>
                <h1 className={styles.title}>Gurumemo</h1>

                <div>
                    ?????????
                    <LocateCondition
                        word={latitudeState}
                        onChange={handleLatitudeChange}
                    />
                </div>
                <div>
                    ?????????
                    <LocateCondition
                        word={longitudeState}
                        onChange={handleLongitudeChange}
                    />
                </div>
                <div>
                    ??????(m)???
                    <LocateCondition word={rangeState} onChange={handleRangeChange} />
                </div>
                <div>
                    ???????????????
                    <LocateCondition word={limitState} onChange={handleLimitChange} />
                </div>
                <SearchButton onClick={searchButtonClick} />
                <YelpResult result={resultState} />
            </main>

            <footer className={styles.footer}>
                <a
                    href="https://vercel.com?utm_source=create-next-app&utm_medium=default-template&utm_campaign=create-next-app"
                    target="_blank"
                    rel="noopener noreferrer"
                >
                    Powered by{" "}
                    <span className={styles.logo}>
                        <Image src="/vercel.svg" alt="Vercel Logo" width={72} height={16} />
                    </span>
                </a>
            </footer>
        </div>
    );
};

export default Home;
