import React from "react";
import { Composition } from "remotion";
import { MainComposition } from "./Composition";

export const Root = () => {
  return (
    <Composition
      id="Main"
      component={MainComposition}
      durationInFrames={3000} // Total sum of Series durationInFrames (750+900+750+600)
      fps={30}
      width={1920}
      height={1080}
    />
  );
};
