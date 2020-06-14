import marked from "marked";
import React, { useMemo } from "react";

export default (source: string) => {
  return useMemo(() => {
    const markedSource = marked(source);
    const markedHtml = { __html: markedSource };
    return <div dangerouslySetInnerHTML={markedHtml} />;
  }, [source]);
};
