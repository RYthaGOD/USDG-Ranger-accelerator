import React from "react";
import { AbsoluteFill, Series, Img, Audio, staticFile, useCurrentFrame, useVideoConfig, interpolate } from "remotion";

const Slide = ({ image, audio }: { image: string; audio: string }) => {
  const frame = useCurrentFrame();
  const { durationInFrames } = useVideoConfig();

  // Smooth fade-in on start
  const opacity = interpolate(frame, [0, 15], [0, 1], {
    extrapolateLeft: "clamp",
    extrapolateRight: "clamp",
  });

  // Subtle Ken Burns zooming effect for static slides
  const scale = interpolate(frame, [0, durationInFrames], [1, 1.03], {
    extrapolateLeft: "clamp",
    extrapolateRight: "clamp",
  });

  return (
    <AbsoluteFill style={{ backgroundColor: '#0A192F', opacity }}>
      <AbsoluteFill 
        style={{ 
          display: 'flex', 
          justifyContent: 'center', 
          alignItems: 'center',
          transform: `scale(${scale})`,
          transition: 'transform 0.1s ease-out'
        }}
      >
        <Img 
          src={staticFile(image)} 
          style={{ 
            width: '100%', 
            height: '100%', 
            objectFit: 'contain'
          }} 
        />
      </AbsoluteFill>
      <Audio src={staticFile(audio)} />
    </AbsoluteFill>
  );
};

export const MainComposition = () => {
  return (
    <Series>
      {/* 1. Optimized Yield with Delta-Neutral Hedging */}
      <Series.Sequence durationInFrames={750}>
        <Slide image="infographic1.png" audio="voice1.wav" />
      </Series.Sequence>

      {/* 2. The Operation (How it Works) */}
      <Series.Sequence durationInFrames={900}>
        <Slide image="infographic2.png" audio="voice2.wav" />
      </Series.Sequence>

      {/* 3. Risk Management & Safety */}
      <Series.Sequence durationInFrames={750}>
        <Slide image="infographic3.png" audio="voice3.wav" />
      </Series.Sequence>

      {/* 4. Production Viability */}
      <Series.Sequence durationInFrames={600}>
        <Slide image="infographic4.png" audio="voice4.wav" />
      </Series.Sequence>
    </Series>
  );
};
