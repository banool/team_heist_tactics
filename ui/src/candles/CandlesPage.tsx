import React, { useEffect } from "react";
import { useDispatch, useSelector } from "react-redux";
import { Link } from "react-router-dom";

import { fetchCandles } from "./api";
import { candlesSelector } from "./slice";
import { Candle } from "./types";

//import CandleForm from "./CandleForm";

import { CandleRoute, pathFor } from "../common/routes";

type CandleRowProps = { candle: Candle };
const CandleRow = ({ candle }: CandleRowProps) => {
  return (
    <tr>
      <td>
        <Link to={pathFor(CandleRoute, { id: candle.id })}>{candle.id}</Link>
      </td>
      <td>{candle.name}</td>
      <td>{candle.notes}</td>
    </tr>
  );
};

type CandlesTableProps = { candles: Candle[] };
const CandlesTable = ({ candles }: CandlesTableProps) => {
  const styles = {
    candlesTable: {
      border: "2px solid #333",
      width: "100%"
    },
    candlesTableHead: {
      border: "2px solid #333"
    }
  };
  return (
    <table style={styles.candlesTable}>
      <thead>
        <tr>
          <th style={styles.candlesTableHead}>id</th>
          <th style={styles.candlesTableHead}>name</th>
          <th style={styles.candlesTableHead}>notes</th>
        </tr>
      </thead>
      <tbody>
        {candles.map(candle => (
          <CandleRow candle={candle} key={candle.id} />
        ))}
      </tbody>
    </table>
  );
};

type CandlesPageProps = {};
const CandlesPage = ({}: CandlesPageProps) => {
  const dispatch = useDispatch();
  const candles = useSelector(candlesSelector);

  useEffect(() => {
    dispatch(fetchCandles());
  }, [dispatch]);

  const renderCandles = () => {
    if (candles.length === 0) {
      return "No candles :(";
    }

    return <CandlesTable candles={candles} />;
  };

  return (
    <div>
      <h2>Candles</h2>
      {renderCandles()}
      {/*}
      <h3>Create</h3>
      <CandleForm />
      */}
    </div>
  );
};

export default CandlesPage;
